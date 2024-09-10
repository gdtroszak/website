---
title: SQLite migrations with Atlas on Fly.io
meta_description: How to perform SQLite migrations with Atlas on Fly.io. Specifically with a Deno application, but the process should be language agnostic.
---

# SQLite migrations with Atlas on Fly.io

*2024-09-08*

I've recently been on a bit of a quest to find the simplest, most boring,
infrastructure for deploying a webservice, and it seems pretty difficult to beat
some compute (a Fly Machine in this case) and a SQLite database. I also like the
idea of declaratively defining my database schema and just having a tool (Atlas)
that makes it so.

The interesting part about this is that because the SQLite database has to be
stored in a volume attached to the Fly Machine, you can't run migrations as a
seperate task like you normally would with an RDBMS that runs as a server (like
MySQL or Postgres). Instead, the migrations need to be run when the machine
starts up. The obvious downside of this is that you're hosed if a migration
takes a long time to complete and you need to serve requests. Services just
getting off the ground shouldn't have that problem, so let's not worry about
problems we don't have. Consider yourself lucky if you do.

## Setting up Atlas

### Define your schema

Atlas let's you define your schema using their HCL language, plain old SQL, or
via on ORM. I'm a luddite, so I chose SQL.

```sql
-- schema.sql

CREATE TABLE customers (
    id INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);
```

If you already have a database, Atlas can inspect it and generate this file
for you.

### Apply your change

Let's apply that change to a non-existent database (that will live at `.data/
db`). Because I'm using SQL to define my schema, I need to provide a `--dev-url`
to a database that Atlas can use process and validate stuff. You can read more
about it [here](https://atlasgo.io/concepts/dev-database). We can just use an
in-memory SQLite database for this.

```sh
atlas schema apply \
  --url "sqlite://.data/db" \
  --to "file://schema.sql" \
  --dev-url "sqlite://dev?mode=memory"
```

Run that and we're presented with the planned changes, which I'll apply. Feel
free to use the `--auto-approve` flag if you don't want to intervene.

```sh
-- Planned Changes:
-- Create "customers" table
CREATE TABLE `customers` (
  `id` integer NULL,
  `first_name` text NOT NULL,
  `last_name` text NOT NULL,
  PRIMARY KEY (`id`)
);
? Are you sure?:
  ▸ Apply
    Lint and edit (requires login)
    Abort
```

## Running the migrations on Fly.io

As previously mentioned, because the SQLite database has to be stored in a
volume attached to the Fly Machine, migrations need to be run when the machine
starts up.

### Setting up Fly.io

Let's take a look at some of the relevant sections of our `fly.toml`. I've added
comments describing the relevance of each line.

```toml
# fly.toml

...

[processes]
  # Command invoked to start the application. We'll get back to this later.
  app = 'task start'

...

[mounts]
  # The name of the volume to mount.
  source = "data"
  # The path the volume will be mounted at in the container.
  destination = "/data"

...

[env]
  # An environment variable the application can use to connect to the database.
  DB_PATH = '/data/db'

...
```

And here's the Dockerfile for the application image. I'm using Deno, but that's
somewhat irrelevant to this approach. Again, comments added to the relevant
lines.

```dockerfile
ARG DENO_VERSION=1.46.3
ARG BIN_IMAGE=denoland/deno:bin-${DENO_VERSION}

# Deno binary layer
FROM ${BIN_IMAGE} AS bin

# Atlas binary layer
FROM arigaio/atlas:latest-alpine as atlas

# Runtime layer
FROM frolvlad/alpine-glibc:alpine-3.13

# Make sure SQLite library is installed
RUN apk --no-cache add ca-certificates sqlite-libs

RUN addgroup --gid 1000 deno \
  && adduser --uid 1000 --disabled-password deno --ingroup deno \
  && mkdir /deno-dir/ \
  && chown deno:deno /deno-dir/

ENV DENO_DIR /deno-dir/
ENV DENO_INSTALL_ROOT /usr/local

ARG DENO_VERSION
ENV DENO_VERSION=${DENO_VERSION}

# Copy deno binary from the Deno binary layer
COPY --from=bin /deno /bin/deno
# Copy atlas binary from the Atlas binary layer
COPY --from=atlas /atlas /bin/atlas

WORKDIR /deno-dir
COPY . .

# (Deno specific) Cache Deno dependencies so they aren't redownloaded everytime
# the machine starts.
RUN /bin/deno cache ./src/app.tsx

# Use deno as the entrypoint. If you look back at the `[processes]` section
# of `fly.toml` you'll notice the `task start` bit. Combine that with this
# entrypoint, and `bin/deno task start` is invoked when the container starts up.
ENTRYPOINT ["/bin/deno"]
CMD ["run", "--allow-net", "https://deno.land/std/examples/echo_server.ts"]
```

### Running the migrations and starting up the application

So when the container starts, it invokes the command `deno task start`. Here's
what that looks like in `deno.json`.

```json
...
"tasks": {
  "migrate": "deno run -RWE --allow-run migrate.ts",
  "serve": "deno serve -RWNE --allow-ffi --unstable-ffi --port=8080 src/app.tsx",
  "start": "deno task migrate && deno task serve"
},
...
```

The `start` task first invokes the `migrate` task, and *if it succeeds* invokes
the `serve` task. Note that the `migrate` task uses a dedicated `migrate.ts`
script. This is purely a convenience to easily access the `DB_PATH` environment
variable set earlier in `fly.toml`.

```typescript
// DB_PATH is loaded into configuration, which is then used by scipts and the
// application.
import { config } from "@/config.ts";

const command = new Deno.Command("atlas", {
  args: [
    "schema",
    "apply",
    "--url",
    `sqlite://${config.db.path}`,
    "--to",
    "file://schema.sql",
    "--dev-url",
    "sqlite://dev?mode=memory",
    "--auto-approve",
  ],
});

const { code, stdout, stderr } = await command.output();

const decoder = new TextDecoder("utf-8");
if (code === 0) {
  console.log("Migration applied successfully.");
  console.log(decoder.decode(stdout));
} else {
  console.log("Migration failed.");
  console.log(decoder.decode(stderr));
}
```

### Deploying it

Running `fly deploy` should deploy everything, but Fly.io will spin up 2
machines by default. That won't work since you can only attach one machine to a
volume. You'll need to run `fly scale count 1` to spin down the second machine.

## Making a change

Assuming you've kept the schema of your local SQLite DB in sync with the one on
Fly.io, you should simply be able to change `schema.sql` and deploy the change.
If they've gotten out of sync, get yourself to the last deployed commit, create
a fresh SQLite DB, then give the schema change a shot. Or it you want to be
*extra* sure about a migration, you can always use `fly ssh sftp` to retrieve
a copy of the actual SQLite database and try it locally before deploying your
change.

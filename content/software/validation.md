---
title: validation
---

# validation

There are a few different forms of validation:

- Syntactic
- Semantic
- Pragmatic

**Syntactic** validation ensures that you are receiving input that is well-formed.
If I have a function that takes a name and prints `Hello ${name}`, `name` better
be a string (or something that can be coerced into a string).

```python
def say_hello(name):
    if not isinstance(name, str):
        raise TypeError("Name must be a string")
    print(f"Hello {name}")

# This will raise a TypeError
say_hello(123)
```

**Semantic** validation ensures that input has meaning. Suppose our program is rude
and only says hello to people we know. We'd need to check that we know them
before saying hello and throw an error if we don't. If we don't know them, they
are meaningless to our program (so rude).

```python
known_names = {"Alice", "Bob", "Charlie"}

def say_hello(name):
	# Syntactic
    if not isinstance(name, str):
        raise TypeError("Name must be a string")
	# Semantic
    if name not in known_names:
        raise ValueError(f"We don't know {name}")
    print(f"Hello {name}")

# This will raise a ValueError because "Dave" is not in known_names
say_hello("Dave")
```

**Pragmatic** validation ensures that the input is appropriate and makes sense given
the current context or state of the application. Imagine our program is supposed
to only greet people we know during business hours. If someone tries to greet
outside these hours, we should reject the input even if it’s syntactically and
semantically valid.

```python
from datetime import datetime

known_names = {"Alice", "Bob", "Charlie"}

def is_business_hours():
    current_hour = datetime.now().hour
    return 9 <= current_hour < 17

def say_hello(name):
    if not isinstance(name, str):
        raise TypeError("Name must be a string")
    if name not in known_names:
        raise ValueError(f"We don't know {name}")
    if not is_business_hours():
        raise ValueError("Cannot greet outside of business hours")
    print(f"Hello {name}")

# This will raise a ValueError if called outside business hours
say_hello("Alice")
```

## where to validate?

Ideally, you should try to keep your core logic focused, which means having
all of this validation there would be awful. Instead, you should use the layers
around your core logic to protect it from this cruft. In an ideal world:

- **Syntactic** validation happens as *close to the edge of your system* as possible (think handlers).
- **Semantic** validation happens in an *orchestration layer* (think services).
- **Pragmatic** validation actually *tends to be the rules your core logic is built
  around, so that is exactly where it belongs* (think domain model).

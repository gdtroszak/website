---
title: logical separation before process separation
meta_description: you need to separate your code logically before introducing a process boundary
---

# logical separation before process separation

It sounds obvious, but I've seen this step skipped so many times. You need to
disentangle your code logically before even talking about "microservices" or
anything similar. A good first step is to think about consistency boundaries.
What data needs to be persisted in one shot? Then start thinking about what
logical concepts belong together.

Also, once you've logically separated, you can introduce a process boundary
without doing all of the infrastructure work to create a microservice. Use
internal events. Run the logic in a separate thread. Whatever it takes to get
into the async paradigm without investing in a ton of infrastructure. Move
to a microservice after doing those things if there is a compelling reason to
(computationally expensive, empower a team to fully own their domain).

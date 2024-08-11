---
title: unit of work
meta_description: TL;DR on the unit of work 
---

# unit of work

Unit of work is a pattern that ensures that software systems transition from
one valid state to another. If several discrete actions must be performed to
transition states, the unit of work ensures that those actions successfully
complete before deeming the state transition successful. If any of them fail, it
ensures the system reverts to the valid state it started in.

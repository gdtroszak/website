---
title: notes on Things You Should Never Do, Part 1
---

# Things You Should Never Do, Part 1

- The single worst strategic mistake a software company can make is to rewrite
  the code from scratch.
- **It's harder to read code than to write it.**
  - This is why programmers always want to throw away the old code and start
    over. This truth makes them always think the old code is a mess.
- New code is never better than old code because the old code has been used.
- When you throw away code and starting from scratch, you are throwing away
  hard-earned knowledge.
- Reasons why code is a mess:
  - Architectural problems. This is solved with careful refactoring.
  - It's inefficient. This is usually relegated to a small part of the project.
    So fix that. 1% or the work gets you 99% of the bang.
  - It's ugly. Refactor it or rename things to make it pretty.
- When you start from scratch, you have no reason to believe that you're going
  to do a better job than you did the first time.
- It's fine to refactor and restructure parts of a program. But never throw away
  the whole thing if it's actually useful.

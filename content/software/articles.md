---
title: software articles
meta_description: good articles on software
---

# software articles

Articles and videos on software that I like.

- [Boundaries](https://www.destroyallsoftware.com/talks/boundaries)

  One of my favorites. Perhaps the best guideline to follow when thinking about
  how to structure a program of any size.
  
- [Functional Core, Imperative Shell](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell/)

  Another one of my favorites. Gives a practical example of using many of the
  concepts outlined in
  [Boundaries](https://www.destroyallsoftware.com/talks/boundaries).

- [Pretty State Machine Patterns in Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)

  I've found myself coming back to this article several times when working
  on some hardware controller logic. Turns out Rust has some incredibly natural
  constructs for encoding state machines.
  
  **TL:DR** Have a generically typed state machine struct. Encode the states in
  structs. Encode the allowed transitions between states using the `From` trait.
  For example,

  ```rust
  // machine
  struct Machine<S> {
    state: S
  }

  // states
  struct State1 {}
  struct State2 {}
  struct State3 {}

  // transitions
  //
  // state 1 -> state 2 -> state 3
  //    ^                     |
  //    |_____________________|

  // transition from state 1 to state 2.
  impl From<Machine<State1>> for Machine<State2> {}

  // transition from state 2 to state 3.
  impl From<Machine<State2>> for Machine<State3> {}

  // transition from state 3 to state 1.
  impl From<Machine<State3>> for Machine<State1> {}
    
  ```

- [Writing Python like it's Rust](https://kobzol.github.io/rust/python/2023/05/20/writing-python-like-its-rust.html)

  Rust forces you to write software in a (arguably) better way. This describes
  some ways to make Python more Rust-like.
  
  **TL;DR** Use type hints in abundance and make illegal states unrepresentable
  in your code.

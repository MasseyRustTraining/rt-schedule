# Realtime Scheduler
Bart Massey 2025

In this exercise you will create a realtime scheduler for a
`no_std` environment. Yes you will: it's not as hard as it
sounds.

A realtime task has a start time and an expected duration:
how long it will take to run. Tasks are fed to the scheduler
as needed, and placed in the task queue.

A realtime environment normally has some integer clock that
"ticks" every so often. The way people often implement a
scheduler for this is quite inefficient, and involves the
following at every timer tick:

* Decrement the start time of every task in the task queue.

* If one or more tasks are at time zero or less, they are
  ready to run: pick at least one and start it.

We will assume that only one task can run at a time in our
system. We will also ignore task priorities, though they are
vitally important for a real realtime scheduler.

---

In `src/lib.rs` is a stub for our first scheduler. Fill in
the types and implementations.

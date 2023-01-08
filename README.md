# Rust Atomics and Locks

## What is atomic operation?
In the context of multi-threading, an operation is atomic if it cannot, from the perspective of other threads, be split into smaller steps.

## Arc (Atomic Reference Counter)
### [How Arc works in Rust](https://medium.com/@DylanKerler1/how-arc-works-in-rust-b06192acd0a6)
- Smart pointer that lets you share immutable data across threads in a thread-safe way
- Need the Arc type when attempting to share data across threads is to ensure that the lifetime of the type that is being shared, lives as long as the longest lasting thread.
- An atomic counter is a type that lets you mutate and increment its value in a thread safe way


## References
- [CPU atomics and orderings explained](https://fy.blackhats.net.au/blog/html/2019/07/16/cpu_atomics_and_orderings_explained.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/basics.html)
# Rust Atomics and Locks

## What is atomic operation?
In the context of multi-threading, an operation is atomic if it cannot, from the perspective of other threads, be split into smaller steps.

## Arc (Atomic Reference Counter)
### [How Arc works in Rust](https://medium.com/@DylanKerler1/how-arc-works-in-rust-b06192acd0a6)
- Smart pointer that lets you share immutable data across threads in a thread-safe way
- Need the Arc type when attempting to share data across threads is to ensure that the lifetime of the type that is being shared, lives as long as the longest lasting thread.
- An atomic counter is a type that lets you mutate and increment its value in a thread safe way

## Tools
- https://godbolt.org/


## References
- [Rust Atomics and Locks](https://marabos.nl/atomics/basics.html)
- [Source](https://github.com/m-ou-se/rust-atomics-and-locks)
- [Blog](https://blog.m-ou.se/rust-standard/)
- [CPU atomics and orderings explained](https://fy.blackhats.net.au/blog/html/2019/07/16/cpu_atomics_and_orderings_explained.html)
- [Explaining Atomics in Rust](https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust/)
- [Atomic vs. Non-Atomic Operations](https://preshing.com/20130618/atomic-vs-non-atomic-operations/)
- [What Every Programmer Should Know About Memory](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf)
- [Spinlocks Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
- [memory order](https://en.cppreference.com/w/cpp/atomic/memory_order)
- [Lock-freedom without garbage collection](https://aturon.github.io/blog/2015/08/27/epoch/#epoch-based-reclamation)

## Blog
- [understanding-memory-ordering](https://www.internalpointers.com/post/understanding-memory-ordering)
- [interacting-with-assembly-in-rust](https://tinkering.xyz/interacting-with-assembly-in-rust/)
- [understanding-memory-ordering](https://www.internalpointers.com/post/understanding-memory-ordering)

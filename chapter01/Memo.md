## Closures
Rust's anonymous functions are called closures. 

```rust
let add_one = |x| {x + 1};

println!("The sum of 5 + 1 is {}", add_one(5));
```

There's one big difference between a closure and named functions, and it's in the name: a closure "closes ovver its environment".
The || syntax means this is an anonymous closure that takes no arguments. 


### Moving closures
Rust has a second type of closure, called a moving closure. 
The difference between a moving closure and an ordinary closure is that a moving closure always takes ownership of all variables that it uses. Ordinary closures, in contrast, just create a reference into the enclosing stack frame. 
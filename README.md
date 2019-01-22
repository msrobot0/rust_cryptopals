Implemenation of 
=======

[cryptopals][https://cryptopals.com] in [Rust[https://doc.rust-lang.org]

### Set 1 Problem 1

This implementation is not optimized. I spent most of the time learning Rust. I originally implemented this with vectors of chars, encountered 'use of moved value' error
switched to String, continued to get the error and then realized that one of my function definitions was not mutable. Towards the end I discovered the chunk method, which I could probably
use to optimize this code. I would also use some bitwise operations instead of looping and 2^... Also this is not Rust like since I use too many conditional not enough pattern matching, but 
its a first step.  The interactive mode does not work and the padding is a hot mess but there it is.

# lib_xch
[![Build Status](https://travis-ci.org/XCH-CEB/lib-xch-ceb.svg?branch=master)](https://travis-ci.org/XCH-CEB/lib-xch-ceb)  

This is crate [xch-ceb](https://crates.io/crates/xch-ceb)'s official lib.  
[lib_xch on crates.io](https://crates.io/crates/lib_xch)  
[lib_xch's documentation](https://docs.rs/lib_xch)  

# Goals of this project
Swift, Small, Safe.  

# Getting Started
First of all, you should pick a version.  
At this time (2018), add following dependency to your `Cargo.toml`:  
```
[dependencies]
lib_xch = "^0.7"
```  
**You can use the latest version number (e.g. v0.8) to replace v0.7**

# Example
For more information, please read the source code of [xch-ceb](https://crates.io/crates/xch-ceb/)
```rust
extern crate lib_xch;

use lib_xch::api::handler::handler_api;
use std::io;

fn main() {
    let equation = input();
    match handler_api::<i32>(&equation) {
        Ok(s) => println!("{:?}", s),
        Err((e, _)) => println!("{:?}", e),
    };
}

// other functions
fn input() -> String {
    println!("[INPUT] Input the equation:");
    let mut equation = String::new();
    io::stdin()
        .read_line(&mut equation)
        .expect("[ERROR] Failed to read line!");
    equation.pop();
    equation
}

```

# License
Licensed under GPL-3.0

# Plans
- [x] Uses regex-based parser
- [x] Uses Gaussian-Jordan Elimination
- [x] Provides the set of Basic Solutions
- [x] Uses AST-based parser
- [ ] Supports WebAssembly

# lib_xch
[![Build Status](https://travis-ci.org/XCH-CEB/xch-project.svg?branch=master)](https://travis-ci.org/XCH-CEB/xch-project)

This is crate [xch-ceb](https://crates.io/crates/xch-ceb)'s official lib.  
[lib_xch on crates.io](https://crates.io/crates/lib_xch)  
[lib_xch's documentation](https://docs.rs/lib_xch)  

# Goals of this project
Swift, Small, Safe.  

# Getting Started
First of all, you should pick a version.  
At this time (2019.1), add following dependency to your `Cargo.toml`:  
```
[dependencies]
lib_xch = "^0.12"
```  

# Example
For more information, please read the source code of [xch-ceb](https://crates.io/crates/xch-ceb/)
```rust
use lib_xch::public::{handler::Handler, structs::ChemicalEquation};
use std::io;

fn main() {
    print_about_info();
    let equ = input();
    match Handler::<i32>::new(&equ).handle() {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{}", e),
    };
}

fn input() -> String {
    println!("[INPUT] Input the equation:");
    let mut equ = String::new();
    io::stdin()
        .read_line(&mut equation)
        .expect("[ERROR] Failed to read line!");
    equ.pop();
    equ
}

```

# License
Licensed under GPL-3.0

# Plans
- [x] Uses regex-based parser
- [x] Uses Gaussian-Jordan Elimination
- [x] Provides the set of Basic Solutions
- [x] Uses AST-based parser

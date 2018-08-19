// Copyright 2017-2018 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// Documentation
//! This is the official library of xch-ceb.
//!
//! It can parse and balance *the Chemical Equation*.
//!
//! -  Unlimited brackets
//! -  No Periodic table of the elements needed
//! -  Provides the set of Basic Solutions instead of only one solution.
//!
//! # Getting Started
//!
//! ```
//! lib_xch::api::handler::handler_api::<i32>("H2O=H2+O2");
//! ```

#![deny(missing_docs)]

// extern crate(s)
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate num;
extern crate regex;

// mods
pub mod api;
mod balancer_mod;
mod math_methods;
mod parser_mod;
mod public;
mod structs;

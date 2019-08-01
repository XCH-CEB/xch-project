// Copyright 2019 LEXUGE
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

//! A type that ensures calculation safety and prevents itself from overflow
//! # Get started
//!
//! ```
//! use scalc::SCell;
//!
//! fn main() {
//!    let a = SCell::<i32>::new(12) * SCell::<i32>::new(3);
//!    assert_eq!(*a.get_data(), 36);
//!
//!    // `error_tag` will be `true` in the presence of overflow behavior(s)
//!    let a = SCell::<i32>::new(std::i32::MAX) + SCell::<i32>::new(1);
//!    assert_eq!(a.is_overflowed(), true);
//!    assert_eq!(*a.get_data(), 1);
//! }
//! ```
//!
//! You can also use [*New Type Idiom*](https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html) in combination of [`derive_more`](https://github.com/JelteF/derive_more) to have better experience.

#![deny(missing_docs)]

#[macro_use]
mod macros;
mod impls;

/// The structure which ensures the calculation safety
#[derive(Copy, Clone, Debug)]
pub struct SCell<T> {
    error_tag: bool,
    data: T,
}

#[cfg(test)]
mod tests {
    use {super::SCell, num_traits::Signed};

    #[test]
    fn overflow_add() {
        let a = SCell::<i32>::new(std::i32::MAX) + SCell::<i32>::new(1);
        assert_eq!(a.is_overflowed(), true);
        assert_eq!(*a.get_data(), 1);
    }

    #[test]
    fn overflow_sub() {
        let a = SCell::<i32>::new(std::i32::MIN) - SCell::<i32>::new(1);
        assert_eq!(a.is_overflowed(), true);
        assert_eq!(*a.get_data(), 1);
    }

    #[test]
    fn overflow_mul() {
        let a = SCell::<i32>::new(std::i32::MAX) * SCell::<i32>::new(2);
        assert_eq!(a.is_overflowed(), true);
        assert_eq!(*a.get_data(), 1);
    }

    #[test]
    fn overflow_div() {
        let a = SCell::<i32>::new(std::i32::MIN) / SCell::<i32>::new(-1);
        assert_eq!(a.is_overflowed(), true);
        assert_eq!(*a.get_data(), 1);
    }

    #[test]
    fn overflow_abs() {
        let a = SCell::<i32>::new(std::i32::MIN).abs();
        assert_eq!(a.is_overflowed(), true);
        assert_eq!(*a.get_data(), 1);
    }
}

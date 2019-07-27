# SCalc
A type that ensures calculation safety and prevents itself from overflow
## Get started
```
use scalc::SCell;

fn main() {
   let a = SCell::<i32>::new(12) * SCell::<i32>::new(3);
   assert_eq!(*a.get_data(), 36);

   // `error_tag` will be `true` in the presence of overflow behavior(s)
   let a = SCell::<i32>::new(std::i32::MAX) + SCell::<i32>::new(1);
   assert_eq!(a.is_overflowed(), true);
   assert_eq!(*a.get_data(), 1);
}
```
You can also use [*New Type Idiom*](https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html) in combination of [`derive_more`](https://github.com/JelteF/derive_more) to have better experience.

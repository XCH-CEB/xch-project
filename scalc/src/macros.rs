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

macro_rules! ops {
    (neg) => {
        impl<T: num_traits::CheckedNeg> std::ops::Neg for SCell<T> {
            type Output = Option<Self>;
            fn neg(self) -> Self::Output {
                self.data.checked_neg().map(SCell::new)
            }
        }
    };
    ($op: tt, $fn: ident, $trait: ident) => {
        paste::item! {
            impl<T: num_traits::[<Checked $trait>]> $trait for SCell<T> {
                type Output = Option<Self>;
                fn $fn(self, rhs: Self) -> Self::Output {
                    self.data.[<checked_ $fn>](&rhs.data).map(SCell::new)
                }
            }
        }
    };
}

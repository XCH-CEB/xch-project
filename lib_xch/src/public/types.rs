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

// Public custom types

// inside use(s)
use super::structs::ChemicalEquation;

// type aliases
pub type DataSet<'a, T> = (&'a ChemicalEquation, Vec<Vec<T>>);
pub(crate) type DataSetIn<'a, T> = (&'a ChemicalEquation, &'a Vec<Vec<T>>); // Use inside for less allocations

// Copyright (C) 2025 Prince Muel
//
// raytracer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// raytracer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with raytracer  If not, see <http://www.gnu.org/licenses/>.

#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix {
    data: [f64; 16],
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool { todo!() }
}

// impl From<&[f64]> for Matrix {
//     fn from(values: &[f64]) -> Self {
//         let size = values.len();
//         let data = values.slic
//         Self { size, data }
//     }
// }
impl From<&[&[f64]]> for Matrix {
    fn from(value: &[&[f64]]) -> Self { todo!() }
}

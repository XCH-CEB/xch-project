// Copyright 2017-2019 LEXUGE
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

use lib_xch::public::{handler::Handler, structs::ChemicalEquation};
use std::io;

fn main() {
    print_about_info();
    let equ = input();
    match Handler::<i32>::new(&equ).handle() {
        Ok((c, v)) => print_ans(&c, &v),
        Err(e) => println!("{}", e),
    };
}

// other functions
fn print_about_info() {
    println!("XCH  Copyright (C) 2017-2019  LEXUGE");
    println!("This program comes with ABSOLUTELY NO WARRANTY;");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions;");
    println!("<> by LEXUGE <LEXUGEyky@outlook.com>");
    println!("License: GPL-3.0-only or GPL-3.0-or-later");
}

fn input() -> String {
    println!("[INPUT] Input the equation:");
    let mut equ = String::new();
    io::stdin()
        .read_line(&mut equ)
        .expect("[ERROR] Failed to read line!");
    equ.pop();
    equ
}

fn print_ans(c: &ChemicalEquation, vecs: &[Vec<&i32>]) {
    println!("[OUTPUT]:");
    for i in 0..c.sum {
        let mut flag = false;
        if vecs.len() == 1 {
            print!("{}", vecs[0][i])
        } else {
            for (index, val) in vecs.iter().enumerate() {
                if (val[i].is_positive()) && flag {
                    print!("+")
                }
                match val[i] {
                    0 => (),
                    1 => {
                        print!("{{k{}}}", index + 1);
                        flag = true
                    }
                    -1 => {
                        print!("-{{k{}}}", index + 1);
                        flag = true
                    }
                    _ => {
                        print!("{}*{{k{}}}", val[i], index + 1);
                        flag = true
                    }
                }
            }
        }
        print!("   ");
    }
    println!();
}

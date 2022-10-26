// #![clippy::msrv = "1.30.0"]
#![allow(clippy::all)]
mod mod_struct;
mod task1;
mod task2;
mod task3;
// cargo clippy --fix
// cargo fmt

// use crate::mod_struct;

fn main() {
    mod_struct::test1();
    task1::test1();
    println!("{}", task2::pig_latin("filter"));
    task3::test3();
}

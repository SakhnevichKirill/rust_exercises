// #![clippy::msrv = "1.30.0"]
#![allow(clippy::all)]
mod mod_struct;
mod task1;
// cargo clippy --fix
// use crate::mod_struct;

fn main() {
    mod_struct::test1();
    task1::test1();
}

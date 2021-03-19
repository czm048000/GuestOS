#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

/*
理想结果：三个程序交替输出 ABC
*/

const WIDTH: usize = 10;
const HEIGHT: usize = 5;

#[no_mangle]
fn main() -> i32 {
    for i in 0..HEIGHT {
        for _ in 0..WIDTH { print!("C"); }
        println!(" [{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    println!("Test write C OK!");
    0
}
#![feature(nll)]

extern crate byteorder;
extern crate webaudio;
extern crate application;
extern crate futures;

mod utils;
mod simple;

fn main() {
    simple::main();
}

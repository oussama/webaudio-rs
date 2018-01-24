extern crate futures;

extern crate stdweb;


mod stdw;


pub use stdw::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
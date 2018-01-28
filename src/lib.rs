extern crate futures;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;


#[cfg(not(target_arch = "wasm32"))]
extern crate rodio;

#[cfg(target_arch = "wasm32")]
mod stdw;
#[cfg(target_arch = "wasm32")]
pub use stdw::*;


#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
extern crate schedulerclient;

#[no_mangle]
pub extern "C" fn div_numbers(a: f64, b: f64) -> f64 {
    schedulerclient::div_numbers(a, b)
}

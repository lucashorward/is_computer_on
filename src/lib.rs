use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    // This is a white-box test in which the test itself tries to assert if the is_computer_on() function return value is equal to the boolean true.
    use crate::is_computer_on;
    #[test]
    fn the_computer_is_on() {
        assert_eq!(is_computer_on(), true);
    }
}

#[wasm_bindgen]
pub fn is_computer_on() -> bool {
    true
}

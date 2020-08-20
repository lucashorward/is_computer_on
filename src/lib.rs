use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    use crate::is_computer_on;
    #[test]
    fn the_computer_is_on() {
        assert_eq!(is_computer_on(), true);
    }
}

#[wasm_bindgen]
pub fn is_computer_on() -> bool {
    return true;
}

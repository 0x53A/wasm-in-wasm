wit_bindgen::generate!({
    world: "calculator",
    path: "../wit"
});

use crate::exports::example::calculator::math::Guest;

struct Calculator;

impl Guest for Calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

export!(Calculator);

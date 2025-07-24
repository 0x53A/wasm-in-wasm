#![no_std]

wit_bindgen::generate!({
    world: "calculator",
    path: "../wit",
});

// Minimal panic handler for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Use wee_alloc as the global allocator for smaller binary size
// IMPORTANT Note: do not use wee_alloc in a production project,
//   the project has been abandoned and there seem to be a few open bugs.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Calculator;

impl exports::example::calculator::math::Guest for Calculator {
    fn add(a: i32, b: i32) -> i32 {
        example::calculator::console::print("in add");
        a + b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        example::calculator::console::print("in multiply");
        a * b
    }
}

export!(Calculator);

mod animation;
mod fourier;

// WASM-JavaScript Interface
#[macro_use]
extern crate lazy_static;

use animation::{naive_animation, Animation};
use fourier::Input;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct JSAnimation<T>
where
    T: Animation,
{
    input: Input,
    animation: T,
}

type Naive = naive_animation::NaiveAnimation;
// TODO: type Fourier = fourier::FourierAnimation;

lazy_static! {
    // Use NaiveAnimation as the default animation.
    static ref ANIMATIONMAP: Mutex<HashMap<usize, JSAnimation<Naive>>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn create(animationId: usize, input: Vec<u32>) {
    console_log!("Initializing animation {}", animationId);
    let input = Input::from(input);
    console_log!("Input: {:?}", input);
    ANIMATIONMAP.lock().unwrap().insert(
        animationId,
        JSAnimation {
            input: Input::from(input),
            animation: Naive::new(),
        },
    );
    console_log!("Created successfully!");
}

#[wasm_bindgen]
pub fn next(animationId: usize) -> JsValue {
    let mut map = ANIMATIONMAP.lock().unwrap();
    let mut jsAnimation = map.get_mut(&animationId).unwrap();
    console_log!("Get animation {}", animationId);
    let frame = jsAnimation.animation.next();
    console_log!("{:?}", frame);
    JsValue::from_serde(&frame).unwrap()
}

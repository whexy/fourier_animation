mod animation;
mod fourier;

// WASM-JavaScript Interface
#[macro_use]
extern crate lazy_static;

use animation::{naive_animation, Animation};
use fourier::Input;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::prelude::*;

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
pub fn init(animationId: usize, input: Vec<u32>) {
    ANIMATIONMAP.lock().unwrap().insert(
        animationId,
        JSAnimation {
            input: Input::from(input),
            animation: Naive::new(),
        },
    );
}

#[wasm_bindgen]
pub fn next(animationId: usize) -> JsValue {
    let mut map = ANIMATIONMAP.lock().unwrap();
    let mut jsAnimation = map.get_mut(&animationId).unwrap();
    let frame = jsAnimation.animation.next();
    JsValue::from_serde(&frame).unwrap()
}

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
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


type Naive = naive_animation::NaiveAnimation;
// TODO: type Fourier = fourier::FourierAnimation;

lazy_static! {
    // TODO: Use Fourier as the default animation.
    static ref ANIMATIONMAP: Mutex<HashMap<usize, Naive>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn create(animationId: usize, input: Vec<f64>) {
    let input = Input::from(input);
    console_log!("{:?}", input);
    ANIMATIONMAP.lock().unwrap().insert(
        animationId,
        Naive::new(Input::from(input)),
    );
}

#[wasm_bindgen]
pub fn next(animationId: usize) -> JsValue {
    let mut map = ANIMATIONMAP.lock().unwrap();
    let mut animation = map.get_mut(&animationId).unwrap();
    let frame = animation.next();
    JsValue::from_serde(&frame).unwrap()
}

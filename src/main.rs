#![allow(unused_variables, unused_imports, dead_code)]

// This is just to compile stuff to wasm.
use wasm_bindgen::prelude::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use std::collections::HashMap;
use std::collections::VecDeque;

#[wasm_bindgen(start)]
pub fn main() {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let canvas = document
    .create_element("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .unwrap();
  document.get_element_by_id("$main_game").unwrap().append_child(&canvas).unwrap();
  canvas.set_width(1000);
  canvas.set_height(1000);
  canvas.style().set_property("border", "solid").unwrap();
  canvas.style().set_property("width", format!("{}px", 1000).as_str()).unwrap();
  canvas.style().set_property("height", format!("{}px", 1000).as_str()).unwrap();
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  let context = Rc::new(context);

  web_sys::console::log_1(&"from rust".into());

  {
    let context = context.clone();
    // From https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
      // This is the raF frame callback:
      context.set_fill_style(&"red".into());



      // Toggling this paint makes the difference between a very busy gpu and an idle gpu...
      context.fill_rect(210.0 + 160.0 - 310.0, 5.0 + 50.0 + 5.0 + (((1 + 5*3 + 1) as f64 * 32.0) / 2.0), 600.0, 50.0);



      request_animation_frame(f.borrow().as_ref().unwrap());
      return;
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
  }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  web_sys::window().expect("no global `window` exists")
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

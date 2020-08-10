#[macro_export]
macro_rules! clog {
    ($($e:expr),*)=>{web_sys::console::log(&{
        let arr=js_sys::Array::new();
        $(
            arr.push(&wasm_bindgen::JsValue::from($e));
        )*
        arr
    })}
}

#[derive(Clone, Debug)]
pub(crate) struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub trait Isomorphism {
    fn isomorphisms(&self, other: &Self) -> bool;
}
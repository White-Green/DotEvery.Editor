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
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn encloses(&self, x: f64, y: f64) -> bool {
        self.x < x && x < self.x + self.w && self.y < y && y < self.y + self.y + self.h
    }

    pub fn center(&self) -> (f64, f64) {
        (self.x + self.w / 2f64, self.y + self.h / 2f64)
    }
}

pub trait Isomorphism {
    fn isomorphisms(&self, other: &Self) -> bool;
}
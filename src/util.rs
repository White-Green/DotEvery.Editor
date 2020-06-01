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
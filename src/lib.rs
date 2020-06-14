use wasm_bindgen::prelude::*;
use yew::prelude::*;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::Window;
use web_sys::HtmlElement;
use web_sys::HtmlAnchorElement;
use web_sys::Document;
use web_sys::Exception;
use crate::program_module::program_module_list::{ProgramModuleListProperties, ProgramModuleList};
use crate::program_module::program_module::{ProgramModuleProperties, ProgramModuleOption, ProgramModuleChildItems};
use crate::dotevery_editor::{DotEveryEditorProperties, DotEveryEditor};

extern crate wee_alloc;

#[macro_use]
mod util;
mod program_module;
mod dotevery_editor;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    clog!("Hello,wasm world!");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(entry) = document.get_element_by_id("app") {
        let props = DotEveryEditorProperties::new(
            ProgramModuleListProperties::new(
                vec![
                    ProgramModuleProperties::new(
                        vec![
                            ProgramModuleOption::StringSign("VariableDefinition".to_string()),
                            ProgramModuleOption::ProgramModule(
                                Some(ProgramModuleProperties::new(
                                    vec![
                                        ProgramModuleOption::StringInput("System.Int32".to_string()),
                                    ],
                                    ProgramModuleChildItems::None))),
                        ],
                        ProgramModuleChildItems::None),
                    ProgramModuleProperties::new(
                        vec![
                            ProgramModuleOption::StringSign("VariableDefinition".to_string()),
                            ProgramModuleOption::ProgramModule(None),
                        ],
                        ProgramModuleChildItems::None)
                ]));
        App::<DotEveryEditor>::new().mount_with_props(entry, props);
    } else {
        clog!("entry point element is not found.");
    }
}

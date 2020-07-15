extern crate wee_alloc;

use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Exception;
use web_sys::HtmlAnchorElement;
use web_sys::HtmlElement;
use web_sys::Window;
use yew::prelude::*;

use crate::components::dotevery_editor::{DotEveryEditorComponent, DotEveryEditorProperties};
use crate::logic::dotevery_editor::DotEveryEditor;
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::logic::program_module_list::ProgramModuleList;


#[macro_use]
mod util;
mod components;
mod logic;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    clog!("Hello,wasm world!");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(entry) = document.get_element_by_id("app") {
        let props = DotEveryEditor::new(
            ProgramModuleList::new(
                vec![
                    ProgramModule::new(
                        vec![
                            ProgramModuleOption::StringSign("VariableDefinition".to_string()),
                            ProgramModuleOption::ProgramModule(
                                Some(ProgramModule::new(
                                    vec![
                                        ProgramModuleOption::StringInput("System.Int32".to_string()),
                                    ],
                                    ProgramModuleChildItems::None))),
                        ],
                        ProgramModuleChildItems::None),
                    ProgramModule::new(
                        vec![
                            ProgramModuleOption::StringSign("VariableDefinition".to_string()),
                            ProgramModuleOption::ProgramModule(None),
                        ],
                        ProgramModuleChildItems::None)
                ]));
        App::<DotEveryEditorComponent>::new().mount_with_props(entry, DotEveryEditorProperties { dotevery_editor: props });
    } else {
        clog!("entry point element is not found.");
    }
}

extern crate wee_alloc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use dotevery_editor_lib::clog;
use dotevery_editor_lib::components::dotevery_editor::{DotEveryEditorComponent, DotEveryEditorProperties};
use dotevery_editor_lib::logic::dotevery_editor::DotEveryEditor;
use dotevery_editor_lib::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};
use dotevery_editor_lib::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use dotevery_editor_lib::logic::program_module_list::ProgramModuleList;

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
        App::<DotEveryEditorComponent<Controller>>::new().mount_with_props(entry, DotEveryEditorProperties::create(props));
    } else {
        clog!("entry point element is not found.");
    }
}

struct Controller { command: Callback<DotEveryEditorCommand> }

impl DotEveryEditorController for Controller {
    fn create(command: Callback<DotEveryEditorCommand>) -> Self where Self: Sized {
        clog!("Controller created");
        Self { command }
    }

    fn update(&mut self, command_id: Uuid, data: DotEveryEditor) {
        clog!(format!("update {:?}",data));
    }
}

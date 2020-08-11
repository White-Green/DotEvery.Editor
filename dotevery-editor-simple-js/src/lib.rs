extern crate wee_alloc;


use uuid::Uuid;
use wasm_bindgen::__rt::std::sync::{Arc, RwLock};
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
        App::<DotEveryEditorComponent<Controller>>::new().mount(entry);
    } else {
        clog!("entry point element is not found.");
    }
}

struct Controller {
    command: Callback<DotEveryEditorCommand>,
    data: Arc<RwLock<DotEveryEditor>>,
    palette: Arc<RwLock<Vec<ProgramModule>>>,
}

impl DotEveryEditorController for Controller {
    fn create(command: Callback<DotEveryEditorCommand>, data: Arc<RwLock<DotEveryEditor>>, palette: Arc<RwLock<Vec<ProgramModule>>>) -> Self where Self: Sized {
        clog!("Controller created");
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
        let palette_data = vec![
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
        ];
        *data.write().unwrap() = props;
        *palette.write().unwrap() = palette_data;
        Self {
            command,
            data,
            palette,
        }
    }

    fn update(&mut self) {
        clog!(format!("update data"));
    }
}

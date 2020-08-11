









#[macro_use]
pub mod util;
pub mod components;
pub mod logic;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen(start)]
// pub fn run_app() {
//     clog!("Hello,wasm world!");
//     let window = web_sys::window().unwrap();
//     let document = window.document().unwrap();
//
//     if let Some(entry) = document.get_element_by_id("app") {
//         let props = DotEveryEditor::new(
//             ProgramModuleList::new(
//                 vec![
//                     ProgramModule::new(
//                         vec![
//                             ProgramModuleOption::StringSign("VariableDefinition".to_string()),
//                             ProgramModuleOption::ProgramModule(
//                                 Some(ProgramModule::new(
//                                     vec![
//                                         ProgramModuleOption::StringInput("System.Int32".to_string()),
//                                     ],
//                                     ProgramModuleChildItems::None))),
//                         ],
//                         ProgramModuleChildItems::None),
//                     ProgramModule::new(
//                         vec![
//                             ProgramModuleOption::StringSign("VariableDefinition".to_string()),
//                             ProgramModuleOption::ProgramModule(None),
//                         ],
//                         ProgramModuleChildItems::None)
//                 ]));
//         App::<DotEveryEditorComponent<i32>>::new().mount_with_props(entry, DotEveryEditorProperties { dotevery_editor: props });
//     } else {
//         clog!("entry point element is not found.");
//     }
// }
//
// impl DotEveryEditorController for i32 {
//     fn create(_command: Callback<DotEveryEditorCommand>) -> Self where Self: Sized {
//         unimplemented!()
//     }
//
//     fn update(&mut self, _command_id: Uuid, _data: DotEveryEditor) {
//         unimplemented!()
//     }
// }
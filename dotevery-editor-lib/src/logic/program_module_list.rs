// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
//
// use crate::logic::dotevery_editor::{DotEveryEditorErrorMessage, DotEveryEditorResult};
// use crate::logic::program_module::ProgramModule;
// use crate::util::Isomorphism;
//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub struct ProgramModuleList<Type: 'static> {
//     pub(crate) id: Uuid,
//     pub(crate) parent: Option<Uuid>,
//     pub(crate) children: Vec<ProgramModule>,
//     pub t: Type,
//     // rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
// }
//
// impl ProgramModuleList {
//     pub fn new(mut children: Vec<ProgramModule>) -> Self {
//         let id = Uuid::new_v4();
//         for module in &mut children {
//             module.parent = Some(id);
//         }
//         Self {
//             id,
//             parent: None,
//             children,
//             // rect_changed_callback: None,
//         }
//     }
//
//     pub fn add(&mut self, target: Uuid, index: usize, module: ProgramModule) -> DotEveryEditorResult<()> {
//         if self.id == target {
//             if self.children.len() < index {
//                 return Err(DotEveryEditorErrorMessage::IndexOutOfRange);
//             }
//             let mut module = module;
//             module.parent = Some(self.id);
//             self.children.insert(index, module);
//             Ok(())
//         } else {
//             for m in &mut self.children {
//                 match m.add(target, index, module.clone()) {
//                     Ok(_) => return Ok(()),
//                     Err(msg) if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
//                     _ => {}
//                 }
//             }
//             Err(DotEveryEditorErrorMessage::NotFound)
//         }
//     }
//
//     pub fn get_module(&self, id: Uuid) -> DotEveryEditorResult<ProgramModule> {
//         if self.id == id {
//             return Err(DotEveryEditorErrorMessage::ModuleToGetMustBeProgramModule);
//         }
//         for module in &self.children {
//             if let Ok(module) = module.get_module(id) {
//                 return Ok(module);
//             }
//         }
//         Err(DotEveryEditorErrorMessage::NotFound)
//     }
//
//     pub fn remove(&mut self, id: Uuid) -> DotEveryEditorResult<()> {
//         for i in 0..self.children.len() {
//             if self.children[i].id == id {
//                 self.children.remove(i);
//                 return Ok(());
//             }
//             match self.children[i].remove(id) {
//                 Ok(_) => return Ok(()),
//                 Err(msg) if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
//                 _ => {}
//             }
//         }
//         Err(DotEveryEditorErrorMessage::NotFound)
//     }
//
//     pub fn deep_clone(&self) -> Self {
//         let children = self.children.iter().map(ProgramModule::deep_clone).collect();
//         let mut new_list = Self::new(children);
//         for module in &mut new_list.children {
//             module.parent = Some(new_list.id.clone());
//         }
//         new_list
//     }
// }
//
// impl Isomorphism for ProgramModuleList {
//     fn isomorphisms(&self, other: &Self) -> bool {
//         self.children.len() == other.children.len()
//             && self.children.iter().zip(&other.children).all(|(a, b)| a.isomorphisms(b))
//     }
// }
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::Callback;

use crate::logic::dotevery_editor::DotEveryEditor;
use crate::logic::program_module::ProgramModule;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DotEveryEditorCommand {
    Update,
    UpdatePalette,
    // UpdateLocal { data: DotEveryEditor, module_id: Uuid },TODO
}

pub trait DotEveryEditorController {
    fn create(command: Callback<DotEveryEditorCommand>, data: Arc<RwLock<DotEveryEditor>>, palette: Arc<RwLock<Vec<ProgramModule>>>) -> Self where Self: Sized;
    fn update(&mut self);
}
//TODO:
//
use uuid::Uuid;
use yew::Callback;

use crate::logic::dotevery_editor::DotEveryEditor;

pub enum DotEveryEditorCommand {
    Update { data: DotEveryEditor },
    // UpdateLocal { data: DotEveryEditor, module_id: Uuid },TODO
}

pub trait DotEveryEditorController {
    fn create(command: Callback<DotEveryEditorCommand>) -> Self where Self: Sized;
    fn update(&mut self, command_id: Uuid, data: DotEveryEditor);
    // fn update_local(&self, _command_id: Uuid, _data: DotEveryEditor, _module_id: Uuid) {} TODO
}
//TODO:
//
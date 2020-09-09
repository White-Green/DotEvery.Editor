use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use yew::{Bridge, Bridged, Callback};
use yew::agent::HandlerId;

use crate::components::dotevery_editor_agent_bridge::DotEveryEditorAgentBridge;
use crate::components::dotevery_editor_controller_proxy_agent::DotEveryEditorControllerProxyAgent;
use crate::components::DotEveryBridge;
use crate::logic::dotevery_editor::DotEveryEditor;
use crate::logic::program_module::ProgramModule;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DotEveryEditorCommand {
    Update,
    UpdatePalette,
    // UpdateLocal { data: DotEveryEditor, module_id: Uuid },TODO
}

pub trait DotEveryEditorController<Type: 'static + Clone + PartialEq = ()>: 'static + Sized {
    type Input;
    type Output;
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<DotEveryEditorControllerProxyAgent<Self, Type>>> {
        DotEveryEditorControllerProxyAgent::<Self, Type>::bridge(callback)
    }
    fn create(data: Arc<RwLock<DotEveryEditor<Type>>>, palette: Arc<RwLock<Vec<ProgramModule<Type>>>>, bridge: DotEveryEditorAgentBridge<Self, Type>) -> Self;
    fn update(&mut self);
    fn handle_input(&mut self, msg: Self::Input, id: HandlerId);
}

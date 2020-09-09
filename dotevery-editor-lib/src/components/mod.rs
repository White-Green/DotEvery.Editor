use yew::Bridge;

use crate::components::dotevery_editor_agent::DotEveryEditorAgent;
use crate::components::dotevery_editor_agent_bridge::DotEveryEditorAgentBridge;
use crate::components::dotevery_editor_controller_proxy_agent::DotEveryEditorControllerProxyAgent;

pub(crate) mod impl_splitter;
pub(crate) mod dragging_program_module;
pub(crate) mod dotevery_editor_agent;
pub mod dotevery_editor;
pub(crate) mod drag_module_agent;
pub(crate) mod program_module;
pub(crate) mod program_module_list;
pub(crate) mod dotevery_editor_agent_bridge;
pub(crate) mod dotevery_editor_controller_proxy_agent;

pub type DotEveryBridge<Controller, Type = ()> = DotEveryEditorAgentBridge<Controller, Type>;
pub type DotEveryEditorControllerBridge<Controller, Type = ()> = Box<dyn Bridge<DotEveryEditorControllerProxyAgent<Controller, Type>>>;
use std::collections::VecDeque;

use bimap::{BiHashMap, Overwritten};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::__rt::std::collections::{HashMap, HashSet};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::dotevery_editor_agent::DotEveryEditorAgentInputMessage::*;
use crate::components::dotevery_editor_agent::DotEveryEditorAgentOutputMessage::*;
use crate::logic::dotevery_editor::{DotEveryEditor, DotEveryEditorErrorMessage};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::ProgramModule;
use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Rect;

pub(crate) struct DotEveryEditorAgent<Controller: 'static + DotEveryEditorController + Serialize + Deserialize<'static>> {
    link: AgentLink<Self>,
    logic: DotEveryEditor,
    manager: Option<HandlerId>,
    controller: Controller,
}

pub(crate) enum DotEveryEditorAgentMessage {
    Ignore
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DotEveryEditorAgentInputMessage {
    SetMeManager,

    SetRoot(DotEveryEditor),
    Add(Uuid, usize, ProgramModule),
    //src,dest,index
    Copy(Uuid, Uuid, usize),
    Remove(Uuid),
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DotEveryEditorAgentOutputMessage {
    ModuleUpdated(DotEveryEditor)
}

impl<Controller: 'static + DotEveryEditorController + Serialize + Deserialize<'static>> Agent for DotEveryEditorAgent<Controller> {
    type Reach = Context;
    type Message = DotEveryEditorAgentMessage;
    type Input = DotEveryEditorAgentInputMessage;
    type Output = DotEveryEditorAgentOutputMessage;

    fn create(link: AgentLink<Self>) -> Self {
        let controller_callback = link.callback(|_| { Self::Message::Ignore });
        Self {
            link,
            logic: DotEveryEditor::new(ProgramModuleList::new(Vec::new())),
            manager: None,
            controller: Controller::create(controller_callback),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::Ignore => {}
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            SetMeManager => self.manager = Some(id),
            SetRoot(root) => {
                self.logic = root;
                if let Some(manager) = self.manager {
                    self.link.respond(manager, ModuleUpdated(self.logic.clone()));
                }
            }
            Add(id, index, module) => {
                if let Err(err) = self.logic.add(id, index, module) {
                    self.handle_error(err);
                } else {
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.clone()));
                    }
                }
            }
            Copy(src, dest, index) => {
                if let Err(err) = self.logic.copy(src, dest, index) {
                    self.handle_error(err);
                } else {
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.clone()));
                    }
                }
            }
            Remove(id) => {
                if let Err(err) = self.logic.remove(id) {
                    self.handle_error(err);
                } else {
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.clone()));
                    }
                }
            }
        }
    }
}

impl<Controller: DotEveryEditorController + Serialize + Deserialize<'static>> DotEveryEditorAgent<Controller> {
    fn handle_error(&mut self, error: DotEveryEditorErrorMessage) {
        clog!(format!("{:?}", error));
        clog!(format!("{:?}", self.logic));
    }
}
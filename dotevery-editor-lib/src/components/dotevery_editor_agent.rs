use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::dotevery_editor_agent::DotEveryEditorAgentInputMessage::*;
use crate::components::dotevery_editor_agent::DotEveryEditorAgentOutputMessage::*;
use crate::logic::dotevery_editor::{DotEveryEditor, DotEveryEditorErrorMessage};
use crate::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};
use crate::logic::program_module::ProgramModule;
use crate::logic::program_module_list::ProgramModuleList;

pub(crate) struct DotEveryEditorAgent<Controller: 'static + DotEveryEditorController> {
    link: AgentLink<Self>,
    logic: Arc<RwLock<DotEveryEditor>>,
    manager: Option<HandlerId>,
    controller: Controller,
    palette: Arc<RwLock<Vec<ProgramModule>>>,
}

#[derive(Clone)]
pub(crate) enum DotEveryEditorAgentMessage {
    ModuleUpdated,
    MsgFromController(DotEveryEditorCommand),
    Ignore,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DotEveryEditorAgentInputMessage {
    SetMeManager,

    // SetRoot(DotEveryEditor),
    Add(Uuid, usize, ProgramModule),
    //src,dest,index
    Copy(Uuid, Uuid, usize),
    Remove(Uuid),
}

#[derive(Serialize, Deserialize)]
pub enum DotEveryEditorAgentOutputMessage {
    ModuleUpdated(DotEveryEditor),
    PaletteUpdated(Vec<ProgramModule>),
}

impl<Controller: 'static + DotEveryEditorController> Agent for DotEveryEditorAgent<Controller> {
    type Reach = Context;
    type Message = DotEveryEditorAgentMessage;
    type Input = DotEveryEditorAgentInputMessage;
    type Output = DotEveryEditorAgentOutputMessage;

    fn create(link: AgentLink<Self>) -> Self {
        let controller_callback = link.callback(|msg| { Self::Message::MsgFromController(msg) });
        let data = Arc::new(RwLock::new(DotEveryEditor::new(ProgramModuleList::new(Vec::new()))));
        let palette = Arc::new(RwLock::new(Vec::new()));
        Self {
            link,
            logic: Arc::clone(&data),
            manager: None,
            controller: Controller::create(controller_callback, Arc::clone(&data), Arc::clone(&palette)),
            palette: Arc::clone(&palette),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::ModuleUpdated => self.controller.update(),
            Self::Message::MsgFromController(_) => { todo!() }
            Self::Message::Ignore => {}
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            SetMeManager => {
                self.manager = Some(id);
                self.link.respond(id, Self::Output::ModuleUpdated(self.logic.read().unwrap().clone()));
                self.link.respond(id, Self::Output::PaletteUpdated(self.palette.read().unwrap().clone()));
            }
            // SetRoot(root) => {
            //     self.logic = root;
            //     self.call_update(Self::Message::ModuleUpdated);
            //     if let Some(manager) = self.manager {
            //         self.link.respond(manager, ModuleUpdated(self.logic.clone()));
            //     }
            // }
            Add(id, index, module) => {
                let result = self.logic.write().unwrap().add(id, index, module);
                if let Err(err) = result {
                    self.handle_error(err);
                } else {
                    self.call_update(Self::Message::ModuleUpdated);
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                    }
                }
            }
            Copy(src, dest, index) => {
                let result = self.logic.write().unwrap().copy(src, dest, index);
                if let Err(err) = result {
                    self.handle_error(err);
                } else {
                    self.call_update(Self::Message::ModuleUpdated);
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                    }
                }
            }
            Remove(id) => {
                let result = self.logic.write().unwrap().remove(id);
                if let Err(err) = result {
                    self.handle_error(err);
                } else {
                    self.call_update(Self::Message::ModuleUpdated);
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                    }
                }
            }
        }
    }
}

impl<Controller: 'static + DotEveryEditorController> DotEveryEditorAgent<Controller> {
    fn handle_error(&mut self, error: DotEveryEditorErrorMessage) {
        clog!(format!("{:?}", error));
        clog!(format!("{:?}", self.logic));
    }

    fn call_update(&mut self, msg: DotEveryEditorAgentMessage) {
        self.link.callback(move |_| msg.clone()).emit(());
    }
}
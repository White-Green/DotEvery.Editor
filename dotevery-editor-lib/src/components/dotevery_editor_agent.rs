use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::{Bridge, Bridged};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::dotevery_editor_agent::DotEveryEditorAgentInputMessage::*;
use crate::components::dotevery_editor_agent::DotEveryEditorAgentOutputMessage::*;
use crate::components::dotevery_editor_agent_bridge::DotEveryEditorAgentBridge;
use crate::logic::dotevery_editor::{DotEveryEditor, DotEveryEditorErrorMessage, DotEveryEditorOperationIndex};
use crate::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};
use crate::logic::program_module::{ProgramModule, ProgramModuleOption};

// use crate::logic::program_module_list::ProgramModuleList;

pub struct DotEveryEditorAgent<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    link: AgentLink<Self>,
    logic: Arc<RwLock<DotEveryEditor<Type>>>,
    palette: Arc<RwLock<Vec<ProgramModule<Type>>>>,
    manager: Option<HandlerId>,
    controller_proxy: Option<HandlerId>,
    controller: Controller,
}

#[derive(Clone)]
pub enum DotEveryEditorAgentMessage<T> {
    ModuleUpdated,
    MsgFromController(DotEveryEditorCommand),
    ResponseFromController(HandlerId, T),
    Ignore,
}

#[derive(Serialize, Deserialize)]
pub enum DotEveryEditorAgentInputMessage<Type: 'static + Clone + PartialEq, IN> {
    SetMeManager,
    SetMeControllerProxy,
    MsgToController(HandlerId, IN),

    // SetRoot(DotEveryEditor),
    Add(Uuid, DotEveryEditorOperationIndex, ProgramModule<Type>),
    //src,dest,index
    Copy(Uuid, Uuid, DotEveryEditorOperationIndex),
    Remove(Uuid),
    UpdateInput { id: Uuid, index: usize, value: String },
}

#[derive(Serialize, Deserialize)]
pub enum DotEveryEditorAgentOutputMessage<Type: 'static + Clone + PartialEq, OUT> {
    ModuleUpdated(DotEveryEditor<Type>),
    PaletteUpdated(Vec<ProgramModule<Type>>),
    ResponseFromController(HandlerId, OUT),
}

impl<Controller, T> Agent for DotEveryEditorAgent<Controller, T>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq {
    type Reach = Context<Self>;
    type Message = DotEveryEditorAgentMessage<Controller::Output>;
    type Input = DotEveryEditorAgentInputMessage<T, Controller::Input>;
    type Output = DotEveryEditorAgentOutputMessage<T, Controller::Output>;

    fn create(link: AgentLink<Self>) -> Self {
        let data = Arc::new(RwLock::new(DotEveryEditor::new(Vec::new())));
        let palette = Arc::new(RwLock::new(Vec::new()));
        let bridge = DotEveryEditorAgentBridge::<Controller, T>::create(
            link.callback(|msg| Self::Message::MsgFromController(msg)),
            link.callback(|(id, msg)| Self::Message::ResponseFromController(id, msg)),
        );
        Self {
            link,
            logic: Arc::clone(&data),
            palette: Arc::clone(&palette),
            manager: None,
            controller_proxy: None,
            controller: Controller::create(Arc::clone(&data), Arc::clone(&palette), bridge),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::ModuleUpdated => self.controller.update(),
            Self::Message::MsgFromController(msg) => {
                if let Some(manager) = self.manager {
                    match msg {
                        DotEveryEditorCommand::Update => {
                            self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                        }
                        DotEveryEditorCommand::UpdatePalette => {
                            self.link.respond(manager, PaletteUpdated(self.palette.read().unwrap().clone()));
                        }
                    }
                } else {
                    clog!("manager is not found");
                }
            }
            Self::Message::ResponseFromController(id, msg) => {
                if let Some(proxy) = self.controller_proxy {
                    self.link.respond(proxy, Self::Output::ResponseFromController(id, msg));
                }
            }
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
            SetMeControllerProxy => {
                self.controller_proxy = Some(id);
            }
            MsgToController(id, msg) => {
                self.controller.handle_input(msg, id);
            }
            // SetRoot(root) => {
            //     self.logic = root;
            //     self.call_update(Self::Message::ModuleUpdated);
            //     if let Some(manager) = self.manager {
            //         self.link.respond(manager, ModuleUpdated(self.logic.clone()));
            //     }
            // }
            Add(id, index, module) => {
                let result = self.logic.write().unwrap().add(id, index, &module);
                // clog!("add operation");
                if let Err(err) = result {
                    self.handle_error(err);
                } else {
                    // clog!("add operation succeed");
                    self.link.send_message(Self::Message::ModuleUpdated);
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                    } else {
                        clog!("manager is not found");
                    }
                }
            }
            Copy(src, dest, index) => {
                let result = self.logic.write().unwrap().copy(src, dest, index);
                if let Err(err) = result {
                    self.handle_error(err);
                } else {
                    self.link.send_message(Self::Message::ModuleUpdated);
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
                    self.link.send_message(Self::Message::ModuleUpdated);
                    if let Some(manager) = self.manager {
                        self.link.respond(manager, ModuleUpdated(self.logic.read().unwrap().clone()));
                    }
                }
            }
            UpdateInput { id, index, value } => {
                let mut logic = self.logic.write().unwrap();
                if let Ok(module) = logic.get_module_mut(id) {
                    if let Some(ProgramModuleOption::StringInput(s)) = module.options.get_mut(index) {
                        *s = value;
                    }
                }
            }
        }
    }
}

impl<Controller, T> DotEveryEditorAgent<Controller, T>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq {
    fn handle_error(&mut self, error: DotEveryEditorErrorMessage) {
        // clog!(format!("{:?}", error));
        // clog!(format!("{:?}", self.logic));
    }
}
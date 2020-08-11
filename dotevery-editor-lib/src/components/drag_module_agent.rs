use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

use bimap::{BiHashMap, Overwritten};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::{Bridge, Bridged};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::dotevery_editor_agent::{DotEveryEditorAgent, DotEveryEditorAgentInputMessage};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::ProgramModule;
use crate::util::Rect;

pub(crate) struct DragModuleAgent<Controller: 'static + DotEveryEditorController> {
    link: AgentLink<Self>,
    manager_id: Option<HandlerId>,
    root_module_id: Option<Uuid>,
    dragging_component: Option<HandlerId>,
    logic_agent_bridge: Box<dyn Bridge<DotEveryEditorAgent<Controller>>>,
    dragging_module: Option<ProgramModule>,
    dragging_module_offset_x: i32,
    dragging_module_offset_y: i32,
    hovering_module: Option<Uuid>,
    hovering_index: Option<usize>,
    rectangles: HashMap<Uuid, Rect>,
    uuid_map: BiHashMap<Uuid, HandlerId>,
    parent_map: HashMap<Uuid, Uuid>,
    children_map: HashMap<Uuid, HashSet<Uuid>>,
}

pub(crate) enum DragModuleMessage {
    Ignore
}

#[derive(Serialize, Deserialize)]
pub enum DragModuleAgentInputMessage {
    TryStartDrag { offset_x: i32, offset_y: i32, module: ProgramModule },
    EndDrag,
    SetRootId(Uuid),
    SetDraggingComponentId,
    SetMyId(Uuid),
    SetParentId { my_id: Uuid, parent_id: Uuid },
    UpdateMousePosition { x: i32, y: i32 },
    UpdateRect { x: f64, y: f64, w: f64, h: f64 },
    UpdateHoveringIndex(Option<usize>),
}

#[derive(Serialize, Deserialize)]
pub enum DragModuleAgentOutputMessage {
    CreateDragComponent { offset_x: i32, offset_y: i32, module: ProgramModule },
    StartDrag,
    EndDrag,
    UpdateDraggingModulePosition { x: i32, y: i32 },
    LeaveHoveringModule,
    MoveHoveringModule { x: i32, y: i32, module_w: f64, module_h: f64 },
    RequestRegisterUuid,
}

impl<Controller: 'static + DotEveryEditorController> Agent for DragModuleAgent<Controller> {
    type Reach = Context<Self>;
    type Message = DragModuleMessage;
    type Input = DragModuleAgentInputMessage;
    type Output = DragModuleAgentOutputMessage;

    fn create(link: AgentLink<Self>) -> Self {
        let callback = link.callback(|_| Self::Message::Ignore);
        Self {
            link,
            manager_id: None,
            root_module_id: None,
            dragging_component: None,
            logic_agent_bridge: DotEveryEditorAgent::bridge(callback),
            dragging_module: None,
            dragging_module_offset_x: 0,
            dragging_module_offset_y: 0,
            hovering_module: None,
            hovering_index: None,
            rectangles: HashMap::new(),
            uuid_map: BiHashMap::new(),
            parent_map: HashMap::new(),
            children_map: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Self::Message::Ignore => {}
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Self::Input::TryStartDrag { offset_x, offset_y, module } => {
                clog!("TryStartDrag", module.id.to_string());
                if self.dragging_module == None {
                    clog!("StartDrag", module.id.to_string());
                    if let Some(id) = self.manager_id {
                        self.link.respond(id, Self::Output::CreateDragComponent { offset_x, offset_y, module: module.deep_clone() });
                        self.dragging_module = Some(module);
                    }
                }
            }
            Self::Input::EndDrag => {
                if let Some(module) = &self.dragging_module {
                    if let Some(hovering_id) = self.hovering_module {
                        clog!("hovering", hovering_id.to_string());
                        if let Some(index) = self.hovering_index {
                            self.logic_agent_bridge.send(DotEveryEditorAgentInputMessage::Copy(module.id, hovering_id, index));
                            self.logic_agent_bridge.send(DotEveryEditorAgentInputMessage::Remove(module.id));
                            clog!("hovering index", format!("Some({})", index));
                        } else {
                            clog!("hovering index", "None");
                        }
                        if let Some(root) = self.manager_id {
                            self.link.respond(root, DragModuleAgentOutputMessage::EndDrag);
                        } else {
                            clog!("manager id is not found in EndDrag");
                        }
                        if let Some(id) = self.uuid_map.get_by_left(&hovering_id) {
                            self.link.respond(*id, Self::Output::LeaveHoveringModule);
                            self.hovering_module = None;
                            self.hovering_index = None;
                        } else {
                            clog!("Uuid is not found in EndDrag");
                        }
                    }
                    self.dragging_module = None;
                }
            }
            Self::Input::UpdateMousePosition { x, y } => {
                clog!("mousemove", x, y);
                if let Some(module) = &self.dragging_module {
                    if let Some(id) = self.dragging_component {
                        if let Some(dragging_module_rect) = self.rectangles.get(&module.id) {
                            self.link.respond(id, Self::Output::UpdateDraggingModulePosition {
                                x,
                                y,
                            });
                            let hovering = self.get_hovering_module_uuid(x, y, module.id);
                            if self.hovering_module != hovering {
                                if let Some(now_hovering_module) = self.hovering_module {
                                    if let Some(now_hovering_module) = self.uuid_map.get_by_left(&now_hovering_module) {
                                        self.link.respond(*now_hovering_module, Self::Output::LeaveHoveringModule);
                                    } else {
                                        clog!("now hovering module Uuid is not found in UpdateMousePosition");
                                    }
                                }
                                self.hovering_module = hovering;
                            }
                            if let Some(id) = hovering {
                                if let Some(now_hovering_module) = self.uuid_map.get_by_left(&id) {
                                    self.link.respond(*now_hovering_module,
                                                      Self::Output::MoveHoveringModule {
                                                          x,
                                                          y,
                                                          module_w: dragging_module_rect.w,
                                                          module_h: dragging_module_rect.h,
                                                      });
                                } else {
                                    clog!("new hovering module Uuid is not found in UpdateMousePosition");
                                }
                            }
                        } else {
                            clog!("dragging module rect is not found in UpdateMousePosition");
                        }
                    }
                }
            }
            Self::Input::UpdateHoveringIndex(index) => self.hovering_index = index,
            Self::Input::UpdateRect { x, y, w, h } => {
                // clog!(format!("{:?}: {:?}=>{:?}",id,self.rectangles.get(&id),Rect {x, y, w, h,}));
                if let Some(uuid) = self.uuid_map.get_by_right(&id) {
                    self.rectangles.insert(*uuid, Rect {
                        x,
                        y,
                        w,
                        h,
                    });
                }
                // clog!(format!("{:?}",self.rectangles));
            }
            Self::Input::SetMyId(uuid) => {
                self.insert_uuid(uuid, id);
            }
            Self::Input::SetRootId(uuid) => {
                self.insert_uuid(uuid, id.clone());
                self.root_module_id = Some(uuid);
                self.manager_id = Some(id);
            }
            Self::Input::SetParentId { my_id, parent_id } => {
                self.insert_uuid(my_id, id);
                self.parent_map.insert(my_id, parent_id);
                if let Some(set) = self.children_map.get_mut(&parent_id) {
                    set.insert(my_id);
                } else {
                    let mut set = HashSet::new();
                    set.insert(my_id);
                    self.children_map.insert(parent_id, set);
                }
            }
            DragModuleAgentInputMessage::SetDraggingComponentId => self.dragging_component = Some(id),
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        if let Some(uuid) = self.uuid_map.get_by_right(&id) {
            self.rectangles.remove(uuid);
            self.parent_map.remove(uuid);
            self.children_map.remove(uuid);
        }
        self.uuid_map.remove_by_right(&id);
    }
}

impl<Controller: DotEveryEditorController> DragModuleAgent<Controller> {
    fn insert_uuid(&mut self, uuid: Uuid, handler_id: HandlerId) {
        match self.uuid_map.insert(uuid, handler_id) {
            Overwritten::Left(_, r) => {
                clog!("uuid over wrapped");
                self.link.respond(r, DragModuleAgentOutputMessage::RequestRegisterUuid);
            }
            Overwritten::Both((_, r), _) => {
                clog!("uuid over wrapped");
                self.link.respond(r, DragModuleAgentOutputMessage::RequestRegisterUuid);
            }
            _ => {}
        }
        clog!(format!("{:?}",self.uuid_map));
    }

    fn get_hovering_module_uuid(&self, x: i32, y: i32, dragging: Uuid) -> Option<Uuid> {
        //TODO:計算量改善
        if self.root_module_id == None { return None; }
        let mut q = VecDeque::new();
        q.push_back(self.root_module_id.unwrap());
        let mut sender = None;
        while let Some(id) = q.pop_front() {
            if dragging == id { continue; }
            if let Some(rect) = self.rectangles.get(&id) {
                if rect.x <= x as f64 && x as f64 <= rect.x + rect.w &&
                    rect.y <= y as f64 && y as f64 <= rect.y + rect.h {
                    if let Some(_) = self.uuid_map.get_by_left(&id) {
                        sender = Some(id.clone());
                    }
                }
            }
            if let Some(map) = self.children_map.get(&id) {
                for id in map {
                    q.push_back(id.clone());
                }
            }
        }
        sender
    }
}
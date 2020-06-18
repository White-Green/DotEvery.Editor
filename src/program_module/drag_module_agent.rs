use serde::{Deserialize, Serialize};
use yew::agent::{Agent, HandlerId, AgentLink, Context};
use wasm_bindgen::__rt::std::collections::{HashMap, HashSet};
use bimap::{BiHashMap, Overwritten};
use uuid::Uuid;
use std::collections::VecDeque;
use crate::util::Rect;


pub(crate) struct DragModuleAgent {
    link: AgentLink<Self>,
    dragging_module: Option<Uuid>,
    dragging_module_offset_x: i32,
    dragging_module_offset_y: i32,
    hovering_module: Option<Uuid>,
    rectangles: HashMap<Uuid, Rect>,
    uuid_map: BiHashMap<Uuid, HandlerId>,
    parent_map: HashMap<Uuid, Uuid>,
    children_map: HashMap<Uuid, HashSet<Uuid>>,
}

pub(crate) enum DragModuleMessage {}

#[derive(Serialize, Deserialize)]
pub(crate) enum DragModuleAgentInputMessage {
    TryStartDrag { offset_x: i32, offset_y: i32 },
    EndDrag,
    SetMyId(Uuid),
    SetParentId { my_id: Uuid, parent_id: Uuid },
    UpdateMousePosition { x: i32, y: i32 },
    UpdateRect { x: f64, y: f64, w: f64, h: f64 },
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DragModuleAgentOutputMessage {
    StartDrag,
    EndDrag,
    UpdateDraggingModulePosition { x: i32, y: i32 },
    LeaveHoveringModule,
    MoveHoveringModule { x: i32, y: i32, module_w: f64, module_h: f64 },
    RequestRegisterUuid,
}

impl Agent for DragModuleAgent {
    type Reach = Context;
    type Message = DragModuleMessage;
    type Input = DragModuleAgentInputMessage;
    type Output = DragModuleAgentOutputMessage;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            dragging_module: None,
            dragging_module_offset_x: 0,
            dragging_module_offset_y: 0,
            hovering_module: None,
            rectangles: HashMap::new(),
            uuid_map: BiHashMap::new(),
            parent_map: HashMap::new(),
            children_map: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {}
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Self::Input::TryStartDrag { offset_x, offset_y } => {
                if self.dragging_module == None {
                    self.dragging_module_offset_x = offset_x;
                    self.dragging_module_offset_y = offset_y;
                    if let Some(id) = self.uuid_map.get_by_right(&id) {
                        self.dragging_module = Some(id.clone());
                        clog!("Start drag");
                    } else {
                        clog!("Uuid is not found in TryStartDrag");
                    }
                    self.link.respond(id, Self::Output::StartDrag);
                }
            }
            Self::Input::EndDrag => {
                if let Some(id) = self.dragging_module {
                    if let Some(id) = self.uuid_map.get_by_left(&id) {
                        self.link.respond(*id, Self::Output::EndDrag);
                        self.dragging_module = None;
                        clog!("End drag");
                    } else {
                        clog!("Uuid is not found in EndDrag");
                    }
                    if let Some(id) = self.hovering_module {
                        if let Some(id) = self.uuid_map.get_by_left(&id) {
                            self.link.respond(*id, Self::Output::LeaveHoveringModule);
                            self.hovering_module = None;
                        } else {
                            clog!("Uuid is not found in EndDrag");
                        }
                    }
                }
            }
            Self::Input::UpdateMousePosition { x, y } => {
                if let Some(id) = self.dragging_module {
                    let dragging_module_rect = self.rectangles.get(&id).unwrap();
                    if let Some(handler_id) = self.uuid_map.get_by_left(&id) {
                        self.link.respond(*handler_id, Self::Output::UpdateDraggingModulePosition {
                            x: x - self.dragging_module_offset_x,
                            y: y - self.dragging_module_offset_y,
                        });
                        let hovering = self.get_hovering_module_uuid(x, y, id);
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
                        clog!("dragging module Uuid is not found in UpdateMousePosition");
                    }
                }
            }
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
            Self::Input::SetParentId { my_id, parent_id } => {
                self.insert_uuid(my_id, id);
                if self.dragging_module == Some(my_id) {
                    self.link.respond(id, Self::Output::StartDrag);
                }
                self.parent_map.insert(my_id, parent_id);
                if let Some(set) = self.children_map.get_mut(&parent_id) {
                    set.insert(my_id);
                } else {
                    let mut set = HashSet::new();
                    set.insert(my_id);
                    self.children_map.insert(parent_id, set);
                }
            }
        }
    }
}

impl DragModuleAgent {
    fn insert_uuid(&mut self, uuid: Uuid, handler_id: HandlerId) {
        match self.uuid_map.insert(uuid, handler_id) {
            Overwritten::Left(_, r) => { self.link.respond(r, DragModuleAgentOutputMessage::RequestRegisterUuid); }
            Overwritten::Both((_, r), _) => { self.link.respond(r, DragModuleAgentOutputMessage::RequestRegisterUuid); }
            _=>{}
        }
        clog!(format!("{:?}",self.uuid_map));
    }

    fn get_hovering_module_uuid(&self, x: i32, y: i32, dragging: Uuid) -> Option<Uuid> {
        //TODO:計算量改善
        let mut root = dragging;
        while let Some(id) = self.parent_map.get(&root) {
            root = id.clone();
        }

        let mut depth_map = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((root, 0));
        while let Some((node, depth)) = q.pop_front() {
            depth_map.insert(node, depth);
            if let Some(children) = self.children_map.get(&node) {
                for child in children {
                    q.push_back((child.clone(), depth + 1));
                }
            }
        }
        let mut max_depth_node = None;
        for (node, rect) in &self.rectangles {
            if *node == dragging { continue; }
            if (x as f64) < rect.x || rect.x + rect.w < (x as f64) { continue; }
            if (y as f64) < rect.y || rect.y + rect.h < (y as f64) { continue; }

            if let Some(uuid) = &max_depth_node {
                if depth_map.get(uuid).unwrap() < depth_map.get(&node).unwrap() {
                    max_depth_node = Some(node.clone());
                }
            } else {
                max_depth_node = Some(node.clone());
            }
        }
        max_depth_node
    }
}
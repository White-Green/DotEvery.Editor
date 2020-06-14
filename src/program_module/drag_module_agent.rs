use serde::{Deserialize, Serialize};
use yew::agent::{Agent, HandlerId, AgentLink, Context};
use wasm_bindgen::__rt::std::collections::HashMap;

#[derive(Clone, Copy)]
struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub(crate) struct DragModuleAgent {
    link: AgentLink<Self>,
    dragging_module: Option<HandlerId>,
    rectangles: HashMap<HandlerId, Rect>,
}

pub(crate) enum DragModuleMessage {}

#[derive(Serialize, Deserialize)]
pub(crate) enum DragModuleAgentInputMessage {
    TryStartDrag,
    EndDrag,
    UpdateMousePosition { x: i32, y: i32 },
    UpdateRect { x: f64, y: f64, w: f64, h: f64 },
}

#[derive(Serialize, Deserialize)]
pub(crate) enum DragModuleAgentOutputMessage {
    StartDrag,
    EndDrag,
    UpdateMousePosition { x: i32, y: i32 },
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
            rectangles: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {}
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Self::Input::TryStartDrag => {
                if let None = self.dragging_module {
                    self.dragging_module = Some(id);
                    self.link.respond(id, Self::Output::StartDrag);
                }
            }
            Self::Input::EndDrag => {
                if let Some(id) = self.dragging_module {
                    self.link.respond(id, Self::Output::EndDrag);
                    self.dragging_module = None;
                }
            }
            Self::Input::UpdateMousePosition { x, y } => {
                if let Some(id) = self.dragging_module {
                    self.link.respond(id, Self::Output::UpdateMousePosition { x, y });
                }
            }
            Self::Input::UpdateRect { x, y, w, h } => {
                self.rectangles.insert(id, Rect {
                    x,
                    y,
                    w,
                    h,
                });
            }
        }
    }
}
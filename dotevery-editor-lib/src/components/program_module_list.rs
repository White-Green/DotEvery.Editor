
use uuid::Uuid;
use web_sys::Element;
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use std::collections::HashMap;

use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::program_module::{ProgramModuleComponent, ProgramModuleProperties};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Rect;

#[derive(Clone, Properties)]
pub(crate) struct ProgramModuleListProperties {
    pub(crate) program_module_list: ProgramModuleList,
    pub(crate) rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
}

pub(crate) struct ProgramModuleListComponent<Controller: 'static + DotEveryEditorController> {
    link: ComponentLink<Self>,
    props: ProgramModuleListProperties,
    hovering_module: Option<(i32, i32, f64, f64)>,
    hovering_index: Option<usize>,
    drag_module_agent: Box<dyn Bridge<DragModuleAgent<Controller>>>,
    child_rectangles: HashMap<Uuid, Rect>,
    self_ref: NodeRef,
}

pub(crate) enum ProgramModuleListMessage {
    Ignore,
    UpdateSelfRect,
    UpdateChildRect { id: Uuid, rect: Rect },
    MoveHoveringModule { x: i32, y: i32, module_w: f64, module_h: f64 },
    LeaveHoveringModule,
    RegisterUuid,
}

impl<Controller: 'static + DotEveryEditorController> Component for ProgramModuleListComponent<Controller> {
    type Message = ProgramModuleListMessage;
    type Properties = ProgramModuleListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            |msg| match msg {
                DragModuleAgentOutputMessage::MoveHoveringModule { x, y, module_w, module_h } => Self::Message::MoveHoveringModule { x, y, module_w, module_h },
                DragModuleAgentOutputMessage::LeaveHoveringModule => Self::Message::LeaveHoveringModule,
                DragModuleAgentOutputMessage::RequestRegisterUuid => Self::Message::RegisterUuid,
                _ => Self::Message::Ignore
            });
        let mut bridge = DragModuleAgent::bridge(callback);
        if let Some(parent) = props.program_module_list.parent {
            bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: props.program_module_list.id, parent_id: parent });
        } else {
            bridge.send(DragModuleAgentInputMessage::SetMyId(props.program_module_list.id));
        }
        Self {
            link,
            props,
            hovering_module: None,
            hovering_index: None,
            drag_module_agent: bridge,
            child_rectangles: HashMap::new(),
            self_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::UpdateSelfRect => {
                if let Some(element) = self.self_ref.cast::<Element>() {
                    let rect = element.get_bounding_client_rect();
                    if let Some(callback) = &self.props.rect_changed_callback {
                        callback.emit((self.props.program_module_list.id, Rect {
                            x: rect.x(),
                            y: rect.y(),
                            w: rect.width(),
                            h: rect.height(),
                        }));
                    }
                    self.drag_module_agent.send(DragModuleAgentInputMessage::UpdateRect {
                        x: rect.x(),
                        y: rect.y(),
                        w: rect.width(),
                        h: rect.height(),
                    });
                }
                false
            }
            Self::Message::UpdateChildRect { id, rect } => {
                self.child_rectangles.insert(id, rect);
                self.link.send_message(Self::Message::UpdateSelfRect);
                false
            }
            Self::Message::MoveHoveringModule { x, y, module_w, module_h } => {
                self.hovering_module = Some((x, y, module_w, module_h));
                let mut index = self.props.program_module_list.children.len();
                for (i, module) in self.props.program_module_list.children.iter().enumerate().rev() {
                    if let Some(rect) = self.child_rectangles.get(&module.id) {
                        if (y as f64) < rect.y {
                            index = i;
                        }
                    }
                }
                self.hovering_index = Some(index);
                self.drag_module_agent.send(DragModuleAgentInputMessage::UpdateHoveringIndex(Some(index)));
                true
            }
            Self::Message::LeaveHoveringModule => {
                self.hovering_module = None;
                self.hovering_index = None;
                true
            }
            Self::Message::RegisterUuid => {
                //self.drag_module_agent.send(DragModuleAgentInputMessage::SetMyId(self.props.program_module_list.id));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.program_module_list == props.program_module_list { return false; }
        if let Some(parent) = props.program_module_list.parent {
            self.drag_module_agent.send(DragModuleAgentInputMessage::SetParentId { my_id: props.program_module_list.id, parent_id: parent });
        } else {
            self.drag_module_agent.send(DragModuleAgentInputMessage::SetMyId(props.program_module_list.id));
        }
        self.child_rectangles.clear();
        self.props = props;
        true
    }


    fn view(&self) -> Html {
        let options = if let Some(index) = self.hovering_index {
            let mut options = Vec::new();
            for (i, p) in self.props.program_module_list.children.iter().enumerate() {
                if i == index {
                    options.push(program_module_placeholder(100f64));
                } else {
                    options.push(program_module_placeholder_ignore());
                }
                let p = p.clone();
                let p = ProgramModuleProperties {
                    program_module: p,
                    rect_changed_callback: Some(self.link.callback(|(id, rect)| ProgramModuleListMessage::UpdateChildRect { id, rect })),
                };
                options.push(
                    html! {
                        <ProgramModuleComponent<Controller> with p/>
                    }
                );
            }
            if index == self.props.program_module_list.children.len() {
                options.push(program_module_placeholder(100f64));
            } else {
                options.push(program_module_placeholder_ignore());
            }
            options
        } else {
            let mut options = Vec::new();
            for p in &self.props.program_module_list.children {
                options.push(program_module_placeholder_ignore());
                let p = p.clone();
                let p = ProgramModuleProperties {
                    program_module: p,
                    rect_changed_callback: Some(self.link.callback(|(id, rect)| ProgramModuleListMessage::UpdateChildRect { id, rect })),
                };
                options.push(
                    html! {
                        <ProgramModuleComponent<Controller> with p/>
                    }
                );
            }
            options.push(program_module_placeholder_ignore());
            options
        };
        html! {
            <div ref=self.self_ref.clone() class="program_module_list">
                {self.props.program_module_list.id}
                {for options}
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        self.link.send_message(Self::Message::UpdateSelfRect);
    }
}

fn program_module_placeholder(width: f64) -> Html {
    html! {
        <div class="program_module_placeholder_hovered" style={format!("width: {}px;", width)}/>
    }
}

fn program_module_placeholder_ignore() -> Html {
    html! {
        <div class="program_module_placeholder"/>
    }
}
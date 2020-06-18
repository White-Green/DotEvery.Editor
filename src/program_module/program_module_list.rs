use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use web_sys::Element;
use crate::program_module::program_module::{ProgramModuleProperties, ProgramModule};
use crate::program_module::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use uuid::Uuid;
use crate::util::Rect;
use std::collections::HashMap;

pub(crate) struct ProgramModuleList {
    link: ComponentLink<Self>,
    props: ProgramModuleListProperties,
    hovering_module: Option<(i32, i32, f64, f64)>,
    drag_module_agent: Box<dyn Bridge<DragModuleAgent>>,
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

#[derive(Clone, Properties)]
pub(crate) struct ProgramModuleListProperties {
    id: Uuid,
    pub(crate) parent: Option<Uuid>,
    children: Vec<ProgramModuleProperties>,
    rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
}

impl ProgramModuleListProperties {
    pub fn new(children: Vec<ProgramModuleProperties>) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            children,
            rect_changed_callback: None,
        }
    }
}

impl Component for ProgramModuleList {
    type Message = ProgramModuleListMessage;
    type Properties = ProgramModuleListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut props = props;
        let rect_changed_callback = link.callback(|(id, rect)| Self::Message::UpdateChildRect { id, rect });
        for module in &mut props.children {
            module.parent = Some(props.id);
            module.rect_changed_callback = Some(rect_changed_callback.clone());
        }
        let callback = link.callback(
            |msg| match msg {
                DragModuleAgentOutputMessage::MoveHoveringModule { x, y, module_w, module_h } => Self::Message::MoveHoveringModule { x, y, module_w, module_h },
                DragModuleAgentOutputMessage::LeaveHoveringModule => Self::Message::LeaveHoveringModule,
                DragModuleAgentOutputMessage::RequestRegisterUuid => Self::Message::RegisterUuid,
                _ => Self::Message::Ignore
            });
        let mut bridge = DragModuleAgent::bridge(callback);
        if let Some(parent) = props.parent {
            bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: props.id, parent_id: parent });
        } else {
            bridge.send(DragModuleAgentInputMessage::SetMyId(props.id));
        }
        Self {
            link,
            props,
            hovering_module: None,
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
                        callback.emit((self.props.id, Rect {
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
                true
            }
            Self::Message::LeaveHoveringModule => {
                self.hovering_module = None;
                true
            }
            Self::Message::RegisterUuid => {
                self.drag_module_agent.send(DragModuleAgentInputMessage::SetMyId(self.props.id));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }


    fn view(&self) -> Html {
        let options = if let Some((x, y, w, h)) = self.hovering_module {
            if self.props.children.len() == 0 {
                vec![
                    html! {
                        <div class="program_module_placeholder" style={format!("width: {}px; height: {}px;", w, h)}/>
                    }
                ]
            } else {
                let mut options = Vec::new();
                let mut placed_placeholder = false;
                for p in &self.props.children {
                    let rect = &self.child_rectangles[&p.id];
                    if !placed_placeholder && (y as f64) < rect.y {
                        options.push(
                            html! {
                                <div class="program_module_placeholder" style={format!("width: {}px; height: {}px;", w, h)}/>
                            }
                        );
                        placed_placeholder = true;
                    } else {
                        options.push(
                            html! {
                                <div style="width: 0; height: 0;"/>
                            }
                        );
                    }
                    let p = p.clone();
                    options.push(
                        html! {
                            <ProgramModule with p/>
                        }
                    )
                }
                if !placed_placeholder {
                    options.push(
                        html! {
                            <div class="program_module_placeholder" style={format!("width: {}px; height: {}px;", w, h)}/>
                        }
                    );
                } else {
                    options.push(
                        html! {
                            <div style="width: 0; height: 0;"/>
                        }
                    );
                }
                options
            }
        } else {
            let mut options = Vec::new();
            for p in &self.props.children {
                options.push(
                    html! {
                        <div style="width: 0; height: 0;"/>
                    }
                );
                let p = p.clone();
                options.push(
                    html! {
                        <ProgramModule with p/>
                    }
                );
            }
            options.push(
                html! {
                    <div style="width: 0; height: 0;"/>
                }
            );
            options
        };
        html! {
            <div ref=self.self_ref.clone() class="program_module_list">
                {for options}
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Self::Message::UpdateSelfRect);
        }
    }
}
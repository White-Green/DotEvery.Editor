use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement, Element};
use crate::program_module::drag_module_agent::{DragModuleAgent, DragModuleAgentOutputMessage, DragModuleAgentInputMessage};
use uuid::Uuid;
use crate::util::Rect;
use std::collections::HashMap;

pub(crate) struct ProgramModule {
    link: ComponentLink<Self>,
    props: ProgramModuleProperties,
    self_ref: NodeRef,
    options_node_ref: Vec<NodeRef>,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent>>,
    child_rectangles: HashMap<Uuid, Rect>,
    hovering_module: Option<(i32, i32, f64, f64)>,
    dragging: bool,
    element_x: i32,
    element_y: i32,
}

pub(crate) enum ProgramModuleMessage {
    Ignore,
    Drag { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    StartDrag,
    EndDrag,
    MoveHoveringModule { x: i32, y: i32, module_w: f64, module_h: f64 },
    LeaveHoveringModule,
    UpdateMousePosition { x: i32, y: i32 },
    UpdateSelfRect,
    UpdateChildRect { id: Uuid, rect: Rect },
    RegisterUuid,
}

#[derive(Clone)]
pub(crate) enum ProgramModuleOption {
    StringSign(String),
    StringInput(String),
    ProgramModule(Option<ProgramModuleProperties>),
}

#[derive(Clone)]
pub(crate) enum ProgramModuleChildItems {
    None,
    Block(Vec<ProgramModuleProperties>),
    MultiBlock(Vec<Vec<ProgramModuleProperties>>),
}

#[derive(Clone, Properties)]
pub(crate) struct ProgramModuleProperties {
    pub(crate) id: Uuid,
    pub(crate) parent: Option<Uuid>,
    options: Vec<ProgramModuleOption>,
    child: ProgramModuleChildItems,
    pub(crate) rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
}

impl ProgramModuleProperties {
    pub fn new(options: Vec<ProgramModuleOption>, child: ProgramModuleChildItems) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            options,
            child,
            rect_changed_callback: None,
        }
    }
}

impl Component for ProgramModule {
    type Message = ProgramModuleMessage;
    type Properties = ProgramModuleProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            |out: DragModuleAgentOutputMessage|
                match out {
                    DragModuleAgentOutputMessage::StartDrag => Self::Message::StartDrag,
                    DragModuleAgentOutputMessage::EndDrag => Self::Message::EndDrag,
                    DragModuleAgentOutputMessage::UpdateDraggingModulePosition { x, y } => Self::Message::UpdateMousePosition { x, y },
                    DragModuleAgentOutputMessage::MoveHoveringModule { x, y, module_w, module_h } => Self::Message::MoveHoveringModule { x, y, module_w, module_h },
                    DragModuleAgentOutputMessage::LeaveHoveringModule => Self::Message::LeaveHoveringModule,
                    DragModuleAgentOutputMessage::RequestRegisterUuid => Self::Message::RegisterUuid,
                    _ => Self::Message::Ignore,
                }
        );
        let rect_changed_callback = link.callback(|(id, rect)| Self::Message::UpdateChildRect { id, rect });
        let mut props = props;
        for option in &mut props.options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.parent = Some(props.id);
                module.rect_changed_callback = Some(rect_changed_callback.clone());
            }
        }
        if let ProgramModuleChildItems::Block(vec) = &mut props.child {}//TODO:
        let mut bridge = DragModuleAgent::bridge(callback);
        if let Some(parent) = props.parent {
            bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: props.id, parent_id: parent });
        } else {
            bridge.send(DragModuleAgentInputMessage::SetMyId(props.id));
        }
        let options_node_ref = vec![NodeRef::default(); props.options.len()];
        Self {
            link,
            props,
            self_ref: NodeRef::default(),
            options_node_ref,
            drag_module_agent_bridge: bridge,
            child_rectangles: HashMap::new(),
            dragging: false,
            hovering_module: None,
            element_x: 0,
            element_y: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::Drag { mouse_x: x, mouse_y: y } => {
                if !self.dragging {
                    let self_element = self.self_ref.cast::<Element>().unwrap();
                    let rect = self_element.get_bounding_client_rect();
                    self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::TryStartDrag {
                        offset_x: x - rect.x().round() as i32,
                        offset_y: y - rect.y().round() as i32,
                    });
                }
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateMousePosition { x, y });
                false
            }
            Self::Message::NoDrag => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::EndDrag);
                false
            }
            Self::Message::StartDrag => {
                let self_element = self.self_ref.cast::<Element>().unwrap();
                set_all_input_disabled(&self_element, true);
                self.dragging = true;
                true
            }
            Self::Message::EndDrag => {
                let self_element = self.self_ref.cast::<Element>().unwrap();
                set_all_input_disabled(&self_element, false);
                self.dragging = false;
                self.link.send_message(Self::Message::UpdateSelfRect);
                true
            }
            Self::Message::MoveHoveringModule { x, y, module_w, module_h } => {
                self.hovering_module = Some((x, y, module_w, module_h));
                self.link.send_message(Self::Message::UpdateSelfRect);
                true
            }
            Self::Message::LeaveHoveringModule => {
                clog!("leave");
                self.hovering_module = None;
                self.link.send_message(Self::Message::UpdateSelfRect);
                true
            }
            Self::Message::UpdateMousePosition { x, y } => {
                self.element_x = x;
                self.element_y = y;
                true
            }
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
                    self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateRect {
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
            Self::Message::RegisterUuid => {
                clog!("RegisterUuid");
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetMyId(self.props.id));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let options = self.props.options.iter().enumerate().map(
            |(i, o)| match o {
                ProgramModuleOption::StringSign(s) => html! {<span ref=self.options_node_ref[i].clone() class="program_module_option program_module_option_string_sign">{s}</span>},
                ProgramModuleOption::StringInput(s) => html! {<input ref=self.options_node_ref[i].clone() onmousemove=self.link.callback(|e:MouseEvent|{if e.buttons()==1{e.stop_propagation();}Self::Message::Ignore}) class="program_module_option program_module_option_string_input" value={s}/>},
                ProgramModuleOption::ProgramModule(p) => match p {
                    Some(p) => {
                        let p = p.clone();
                        html! {<div ref=self.options_node_ref[i].clone() class="program_module_option program_module_option_module"><ProgramModule with p/></div>}
                    }
                    None => {
                        let mut inner = html! {};
                        if let Some((x, y, w, h)) = self.hovering_module {
                            if let Some(element) = self.options_node_ref[i].cast::<Element>() {
                                let rect = element.get_bounding_client_rect();
                                if rect.x() <= (x as f64) && (x as f64) <= rect.x() + rect.width() &&
                                    rect.y() <= (y as f64) && (y as f64) <= rect.y() + rect.height()
                                {
                                    inner = html! {
                                        <div class="program_module_placeholder" style={format!("width: {}px; height: {}px;", w, h)}/>
                                    };
                                }
                            }
                        }
                        html! {
                            <div ref=self.options_node_ref[i].clone() class="program_module_option program_module_option_module">
                                {inner}
                            </div>
                        }
                    }
                },
            });
        let mouse_move = self.link.callback(|e: MouseEvent| {
            if e.buttons() == 1 {
                Self::Message::Drag {
                    mouse_x: e.page_x(),
                    mouse_y: e.page_y(),
                }
            } else {
                Self::Message::NoDrag
            }
        });
        let style = if self.dragging {
            format!("position:absolute;top:{}px;left:{}px", self.element_y, self.element_x)
        } else { "".to_string() };
        html! {
            <div ref=self.self_ref.clone() style=style class="program_module">
                <div onmousemove=mouse_move class="program_module_options">
                    {for options}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Self::Message::UpdateSelfRect);
        }
    }
}

fn set_all_input_disabled(base: &Element, disabled: bool) {
    let nodes = base.query_selector_all("input").unwrap();
    for i in 0..nodes.length() {
        let node = nodes.get(i).unwrap();
        let input = node.unchecked_ref::<HtmlInputElement>();
        input.set_disabled(disabled);
    }
}
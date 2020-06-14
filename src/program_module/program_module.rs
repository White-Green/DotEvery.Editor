use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlElement, Element};
use crate::program_module::drag_module_agent::{DragModuleAgent, DragModuleAgentOutputMessage, DragModuleAgentInputMessage};

pub(crate) struct ProgramModule {
    link: ComponentLink<Self>,
    props: ProgramModuleProperties,
    self_ref: NodeRef,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent>>,
    dragging: bool,
    drag_offset_x: i32,
    drag_offset_y: i32,
    element_x: i32,
    element_y: i32,
}

pub enum ProgramModuleMessage {
    Ignore,
    Drag { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    StartDrag,
    EndDrag,
    UpdateMousePosition { x: i32, y: i32 },
    UpdateChildRect,
}

#[derive(Clone)]
pub enum ProgramModuleOption {
    StringSign(String),
    StringInput(String),
    ProgramModule(Option<ProgramModuleProperties>),
}

#[derive(Clone)]
pub enum ProgramModuleChildItems {
    None,
    Block(Vec<ProgramModuleProperties>),
    MultiBlock(Vec<Vec<ProgramModuleProperties>>),
}

#[derive(Clone, Properties)]
pub struct ProgramModuleProperties {
    options: Vec<ProgramModuleOption>,
    child: ProgramModuleChildItems,
    pub(crate) rect_changed_callback: Option<Callback<()>>,
}

impl ProgramModuleProperties {
    pub fn new(options: Vec<ProgramModuleOption>, child: ProgramModuleChildItems) -> Self {
        Self {
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
                    DragModuleAgentOutputMessage::UpdateMousePosition { x, y } => Self::Message::UpdateMousePosition { x, y },
                    _ => Self::Message::Ignore,
                }
        );
        let rect_changed_callback = link.callback(|_| Self::Message::UpdateChildRect);
        let mut props = props;
        for option in &mut props.options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.rect_changed_callback = Some(rect_changed_callback.clone());
            }
        }
        if let ProgramModuleChildItems::Block(vec) = &mut props.child {}
        Self {
            link,
            props,
            self_ref: NodeRef::default(),
            drag_module_agent_bridge: DragModuleAgent::bridge(callback),
            dragging: false,
            drag_offset_x: 0,
            drag_offset_y: 0,
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
                    self.drag_offset_x = x - rect.x().round() as i32;
                    self.drag_offset_y = y - rect.y().round() as i32;
                    self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::TryStartDrag);
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
                true
            }
            Self::Message::UpdateMousePosition { x, y } => {
                self.element_x = x - self.drag_offset_x;
                self.element_y = y - self.drag_offset_y;
                true
            }
            Self::Message::UpdateChildRect => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let options = self.props.options.iter().map(
            |o| match o {
                ProgramModuleOption::StringSign(s) => html! {<span class="program_module_option program_module_option_string_sign">{s}</span>},
                ProgramModuleOption::StringInput(s) => html! {<input onmousemove=self.link.callback(|e:MouseEvent|{if e.buttons()==1{e.stop_propagation();}Self::Message::Ignore}) class="program_module_option program_module_option_string_input" value={s}/>},
                ProgramModuleOption::ProgramModule(p) => match p {
                    Some(p) => {
                        let p = p.clone();
                        html! {<div /*onmousemove=self.link.callback(|e:MouseEvent|{e.stop_propagation();Self::Message::Ignore})*/ class="program_module_option program_module_option_module"><ProgramModule with p/></div>}
                    }
                    None => html! {<div /*onmousemove=self.link.callback(|e:MouseEvent|{e.stop_propagation();Self::Message::Ignore})*/ class="program_module_option program_module_option_module"/>}
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
}

fn set_all_input_disabled(base: &Element, disabled: bool) {
    let nodes = base.query_selector_all("input").unwrap();
    for i in 0..nodes.length() {
        let node = nodes.get(i).unwrap();
        let input = node.unchecked_ref::<HtmlInputElement>();
        input.set_disabled(disabled);
    }
}
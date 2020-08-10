

use serde::{Deserialize, Serialize};

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::program_module::{ProgramModuleComponent, ProgramModuleProperties};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::{ProgramModule, ProgramModuleOption};


#[derive(Clone, Properties, PartialEq)]
pub(crate) struct DraggingProgramModuleProperties {
    pub(crate) program_module: ProgramModule,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
    pub(crate) visibility: bool,
}

pub(crate) struct DraggingProgramModuleComponent<Controller: 'static + DotEveryEditorController + Serialize + Deserialize<'static>> {
    link: ComponentLink<Self>,
    props: DraggingProgramModuleProperties,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent<Controller>>>,
    element_x: i32,
    element_y: i32,
}

pub(crate) enum DraggingProgramModuleMessage {
    Ignore,
    Drag { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    EndDrag,
    UpdateMousePosition { x: i32, y: i32 },
}

impl<Controller: 'static + DotEveryEditorController + Serialize + Deserialize<'static>> Component for DraggingProgramModuleComponent<Controller> {
    type Message = DraggingProgramModuleMessage;
    type Properties = DraggingProgramModuleProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            |out: DragModuleAgentOutputMessage|
                match out {
                    DragModuleAgentOutputMessage::EndDrag => Self::Message::EndDrag,
                    DragModuleAgentOutputMessage::UpdateDraggingModulePosition { x, y } => Self::Message::UpdateMousePosition { x, y },
                    _ => Self::Message::Ignore,
                }
        );
        let mut bridge = DragModuleAgent::bridge(callback);
        bridge.send(DragModuleAgentInputMessage::SetDraggingComponentId);
        Self {
            link,
            props,
            drag_module_agent_bridge: bridge,
            element_x: 0,
            element_y: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::Drag { mouse_x: x, mouse_y: y } => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateMousePosition { x, y });
                false
            }
            Self::Message::NoDrag => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::EndDrag);
                false
            }
            Self::Message::EndDrag => {
                true
            }
            Self::Message::UpdateMousePosition { x, y } => {
                self.element_x = x - self.props.offset_x;
                self.element_y = y - self.props.offset_y;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props == props { return false; }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if self.props.visibility {
            let options = self.props.program_module.options.iter().enumerate().map(
                |(i, o)| match o {
                    ProgramModuleOption::StringSign(s) => Self::render_string_sign(s.clone()),
                    ProgramModuleOption::StringInput(s) => Self::render_string_input(
                        self.link.callback(Self::string_input_mousemove),
                        s.clone()),
                    ProgramModuleOption::ProgramModule(p) => self.render_program_module(i, p),
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
            let style = format!("position:absolute;top:{}px;left:{}px;", self.element_y, self.element_x);
            let html: Html = html! {
                <div style=style class="program_module">
                    {self.props.program_module.id}
                    <div onmousemove=mouse_move class="program_module_options">
                        {for options}
                    </div>
                </div>
            };
            html
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, _first_render: bool) {}
}


fn set_all_input_disabled(base: &Element, disabled: bool) {
    let nodes = base.query_selector_all("input").unwrap();
    for i in 0..nodes.length() {
        let node = nodes.get(i).unwrap();
        let input = node.unchecked_ref::<HtmlInputElement>();
        input.set_disabled(disabled);
    }
}

impl<Controller: DotEveryEditorController + Serialize + Deserialize<'static>> DraggingProgramModuleComponent<Controller> {
    fn render_string_sign(s: String) -> Html {
        html! {<span class="program_module_option program_module_option_string_sign">{s}</span>}
    }

    fn render_string_input(onmousemove: Callback<MouseEvent>, value: String) -> Html {
        html! {<input onmousemove=onmousemove class="program_module_option program_module_option_string_input" value=value/>}
    }

    fn string_input_mousemove(e: MouseEvent) -> DraggingProgramModuleMessage {
        if e.buttons() == 1 { e.stop_propagation(); }
        DraggingProgramModuleMessage::Ignore
    }

    fn render_program_module(&self, _i: usize, p: &Option<ProgramModule>) -> Html {
        match p {
            Some(p) => {
                let p = p.clone();
                let p = ProgramModuleProperties {
                    program_module: p,
                    rect_changed_callback: None,
                };
                let html: Html = html! {
                    <div class="program_module_option program_module_option_module">
                        <ProgramModuleComponent<Controller> with p/>
                    </div>
                };
                html
            }
            None => {
                let placeholder = html! {
                        <div class="program_module_option_program_module_placeholder"/>
                    };
                let html: Html = html! {
                    <div class="program_module_option program_module_option_module">
                        {placeholder}
                    </div>
                };
                html
            }
        }
    }
}

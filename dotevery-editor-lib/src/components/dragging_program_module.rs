use either::Either;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::program_module::{ProgramModuleComponent, ProgramModuleComponentImplTypeCanNotDrag, ProgramModuleProperties};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::{ProgramModule, ProgramModuleOption};

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct DraggingProgramModuleProperties<T: 'static + Clone + PartialEq> {
    pub(crate) program_module: ProgramModule<T>,
    pub(crate) offset_x: i32,
    pub(crate) offset_y: i32,
    pub(crate) visibility: bool,
}

pub(crate) struct DraggingProgramModuleComponent<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    link: ComponentLink<Self>,
    props: DraggingProgramModuleProperties<Type>,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent<Controller, Type>>>,
    element_x: i32,
    element_y: i32,
}

pub(crate) enum DraggingProgramModuleMessage {
    Ignore,
    NoDrag,
    UpdateMousePosition { x: i32, y: i32 },
}

impl<Controller, T> Component for DraggingProgramModuleComponent<Controller, T>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq {
    type Message = DraggingProgramModuleMessage;
    type Properties = DraggingProgramModuleProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            |out: DragModuleAgentOutputMessage<T>|
                match out {
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
            Self::Message::NoDrag => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::EndDrag);
                false
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
            let props = ProgramModuleProperties {
                program_module: Either::Left(self.props.program_module.clone()),
                rect_changed_callback: self.link.callback(|_| Self::Message::Ignore),
            };
            let style = format!("position:absolute;top:{}px;left:{}px;", self.element_y, self.element_x);
            let html: Html = html! {
                <div style=style class="program_module_dragging">
                    <ProgramModuleComponent<Controller, T, ProgramModuleComponentImplTypeCanNotDrag> with props/>
                    // {self.props.program_module.id}
                    // <div onmousemove=mouse_move class="program_module_options">
                    //     {for options}
                    // </div>
                </div>
            };
            html
        } else {
            html! {}
        }
    }

    fn rendered(&mut self, _first_render: bool) {}
}

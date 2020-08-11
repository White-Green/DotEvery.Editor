use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::dotevery_editor_agent::{DotEveryEditorAgent, DotEveryEditorAgentInputMessage, DotEveryEditorAgentOutputMessage};
use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::dragging_program_module::{DraggingProgramModuleComponent, DraggingProgramModuleProperties};
use crate::components::program_module_list::{ProgramModuleListComponent, ProgramModuleListProperties};
use crate::logic::dotevery_editor::DotEveryEditor;
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems};
use crate::logic::program_module_list::ProgramModuleList;

#[derive(Clone, Properties, Default)]
pub struct DotEveryEditorProperties {
    // pub(crate) dotevery_editor: DotEveryEditor,
}

impl DotEveryEditorProperties {
    pub fn create() -> Self {
        Self {}
    }
}

pub struct DotEveryEditorComponent<Controller: 'static + DotEveryEditorController> {
    link: ComponentLink<Self>,
    props: DotEveryEditorProperties,
    dragging_component_props: DraggingProgramModuleProperties,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent<Controller>>>,
    logic_agent_bridge: Box<dyn Bridge<DotEveryEditorAgent<Controller>>>,
    logic_data: DotEveryEditor,
    palette_data: Vec<ProgramModule>,
}

pub enum DotEveryEditorMessage {
    Ignore,
    MouseMove { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    SendDragModuleAgentMessage(DragModuleAgentInputMessage),
    OutputFromLogicAgent(DotEveryEditorAgentOutputMessage),
    OutputFromDragModuleAgent(DragModuleAgentOutputMessage),
}

impl<Controller: 'static + DotEveryEditorController> Component for DotEveryEditorComponent<Controller> {
    type Message = DotEveryEditorMessage;
    type Properties = DotEveryEditorProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let drag_module_callback = link.callback(|msg| Self::Message::OutputFromDragModuleAgent(msg));
        let logic_callback = link.callback(|msg| Self::Message::OutputFromLogicAgent(msg));
        let mut logic_agent_bridge = DotEveryEditorAgent::bridge(logic_callback);
        logic_agent_bridge.send(DotEveryEditorAgentInputMessage::SetMeManager);
        // logic_agent_bridge.send(DotEveryEditorAgentInputMessage::SetRoot(props.dotevery_editor.clone()));
        // props.dotevery_editor.list.children.clear();
        let mut drag_module_agent_bridge = DragModuleAgent::bridge(drag_module_callback);
        // drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetRootId(props.dotevery_editor.id.clone()));
        let dragging_component_props = DraggingProgramModuleProperties {
            program_module: ProgramModule::new(Vec::new(), ProgramModuleChildItems::None),
            offset_x: 0,
            offset_y: 0,
            visibility: false,
        };
        Self {
            link,
            props,
            dragging_component_props,
            drag_module_agent_bridge,
            logic_agent_bridge,
            logic_data: DotEveryEditor::new(ProgramModuleList::new(Vec::new())),
            palette_data: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::SendDragModuleAgentMessage(msg) => {
                self.drag_module_agent_bridge.send(msg);
                false
            }
            Self::Message::OutputFromLogicAgent(msg) =>
                match msg {
                    DotEveryEditorAgentOutputMessage::ModuleUpdated(logic) => {
                        if self.logic_data.id != logic.id {
                            self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetRootId(logic.id));
                        }
                        self.logic_data = logic;
                        true
                    }
                    DotEveryEditorAgentOutputMessage::PaletteUpdated(palette) => {
                        self.palette_data = palette;
                        true
                    }
                    _ => false
                }
            Self::Message::OutputFromDragModuleAgent(msg) =>
                match msg {
                    DragModuleAgentOutputMessage::CreateDragComponent { offset_x, offset_y, module } => {
                        self.dragging_component_props = DraggingProgramModuleProperties { offset_x, offset_y, program_module: module, visibility: true };
                        true
                    }
                    DragModuleAgentOutputMessage::EndDrag => {
                        self.dragging_component_props.visibility = false;
                        true
                    }
                    _ => false,
                }
            Self::Message::MouseMove { mouse_x, mouse_y } => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateMousePosition { x: mouse_x, y: mouse_y });
                false
            }
            Self::Message::NoDrag => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::EndDrag);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let list = self.logic_data.list.clone();
        let list = ProgramModuleListProperties {
            program_module_list: list,
            rect_changed_callback: None,
        };
        let dragging = self.dragging_component_props.clone();
        let mouse_move = self.link.callback(|e: MouseEvent| {
            if e.buttons() == 1 {
                Self::Message::MouseMove {
                    mouse_x: e.page_x(),
                    mouse_y: e.page_y(),
                }
            } else {
                Self::Message::NoDrag
            }
        });
        html! {
            <div onmousemove=mouse_move class="dotevery_editor">
                {"DotEvery.Editor"}
                <ProgramModuleListComponent<Controller> with list/>
                <DraggingProgramModuleComponent<Controller> with dragging/>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let window = web_sys::window().unwrap();
            let callback = self.link.callback(|m| m);
            let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
                clog!("mousemove", e.page_x(), e.page_y());
                callback.emit(Self::Message::SendDragModuleAgentMessage(DragModuleAgentInputMessage::UpdateMousePosition { x: e.page_x(), y: e.page_y() }));
                if e.buttons() != 1 {
                    callback.emit(Self::Message::SendDragModuleAgentMessage(DragModuleAgentInputMessage::EndDrag));
                }
            }) as Box<dyn FnMut(_)>);
            if let Err(err) = window.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()) {
                clog!("add mousemove event failed",err);
            }
            closure.forget();
        }
    }
}
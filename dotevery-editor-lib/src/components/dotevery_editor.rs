use std::collections::HashMap;

use either::Either;
use uuid::Uuid;
use wasm_bindgen::__rt::std::collections::VecDeque;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::dotevery_editor_agent::{DotEveryEditorAgent, DotEveryEditorAgentInputMessage, DotEveryEditorAgentOutputMessage};
use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::dragging_program_module::{DraggingProgramModuleComponent, DraggingProgramModuleProperties};
use crate::components::program_module::{get_page_offset, ProgramModuleComponent, ProgramModuleComponentImplTypeDefault, ProgramModuleComponentImplTypeListOnly, ProgramModuleDefault, ProgramModuleProperties};
// use crate::components::program_module_list::{ProgramModuleListComponent, ProgramModuleListProperties};
use crate::logic::dotevery_editor::DotEveryEditor;
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems};
use crate::util::Rect;

// use crate::logic::program_module_list::ProgramModuleList;

#[derive(Clone, Properties, Default)]
pub struct DotEveryEditorProperties {
    // pub(crate) dotevery_editor: DotEveryEditor,
}

impl DotEveryEditorProperties {
    pub fn create() -> Self {
        Self {}
    }
}

pub struct DotEveryEditorComponent<Controller, Type = ()>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    link: ComponentLink<Self>,
    props: DotEveryEditorProperties,
    trash_area_ref: NodeRef,
    dragging_component_props: Option<DraggingProgramModuleProperties<Type>>,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent<Controller, Type>>>,
    logic_agent_bridge: Box<dyn Bridge<DotEveryEditorAgent<Controller, Type>>>,
    logic_data: DotEveryEditor<Type>,
    palette_data: Vec<ProgramModule<Type>>,
}

pub enum DotEveryEditorMessage<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    Ignore,
    MouseMove { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    UpdateChildRect { id: Uuid, rect: Rect },
    SendDragModuleAgentMessage(DragModuleAgentInputMessage<Type>),
    OutputFromLogicAgent(DotEveryEditorAgentOutputMessage<Type, Controller::Output>),
    OutputFromDragModuleAgent(DragModuleAgentOutputMessage<Type>),
}

impl<Controller, T> Component for DotEveryEditorComponent<Controller, T>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq {
    type Message = DotEveryEditorMessage<Controller, T>;
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
        // let dragging_component_props = DraggingProgramModuleProperties {
        //     program_module: ProgramModule::new(Vec::new(), ProgramModuleChildItems::None),
        //     offset_x: 0,
        //     offset_y: 0,
        //     visibility: false,
        // };
        Self {
            link,
            props,
            trash_area_ref: NodeRef::default(),
            dragging_component_props: None,
            drag_module_agent_bridge,
            logic_agent_bridge,
            logic_data: DotEveryEditor::new(Vec::new()),
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
                        // clog!("update logic",format!("{:#?}",logic.list.iter().map(ProgramModule::isomorphic_transform).collect::<Vec<ProgramModule<()>>>()));
                        if self.logic_data.id != logic.id {
                            self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetRootId(logic.id));
                        }
                        // {
                        //     let logic = logic.clone();
                        //     let mut q = logic.list.into_iter().collect::<VecDeque<_>>();
                        //     while let Some(module) = q.pop_front() {
                        //         clog!(format!("{:?}",module.id));
                        //         match module.child {
                        //             ProgramModuleChildItems::BlockVertical(list) | ProgramModuleChildItems::BlockHorizontal(list) => {
                        //                 for m in list {
                        //                     q.push_back(m);
                        //                 }
                        //             }
                        //             _ => {}
                        //         }
                        //     }
                        // }
                        self.logic_data = logic;
                        // self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::Clear);
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
                        self.dragging_component_props = Some(DraggingProgramModuleProperties { offset_x, offset_y, program_module: module, visibility: true });
                        true
                    }
                    DragModuleAgentOutputMessage::EndDrag => {
                        if let Some(component) = &mut self.dragging_component_props {
                            component.visibility = false;
                        }
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
            Self::Message::UpdateChildRect { id, rect } => {
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // clog!("view",format!("{:#?}",self.logic_data.list.iter().map(ProgramModule::isomorphic_transform).collect::<Vec<ProgramModule<()>>>()));
        let dragging = if let Some(dragging) = &self.dragging_component_props {
            let dragging = dragging.clone();
            html! {
                <DraggingProgramModuleComponent<Controller, T> with dragging/>
            }
        } else {
            html! {}
        };
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
        // let mut module = ProgramModule::new_default_with_id(Uuid::nil(), Vec::new(), ProgramModuleChildItems::BlockVertical(self.logic_data.list.iter().map(ProgramModule::deep_clone).collect()));
        // module.parent = Some(self.logic_data.id);
        let module = ProgramModuleProperties {
            program_module: Either::Right(ProgramModuleDefault {
                list: self.logic_data.list.iter().map(|module| {
                    let mut module = module.clone();
                    module.parent = Some(Uuid::nil());
                    module
                }).collect(),
                parent: self.logic_data.id,
            }),
            rect_changed_callback: self.link.callback(|(id, rect)| { Self::Message::UpdateChildRect { id, rect } }),
        };
        let palette = self.palette_data.iter().map(|p| {
            let module = ProgramModuleProperties {
                program_module: Either::Left(p.clone()),
                rect_changed_callback: self.link.callback(|_| { Self::Message::Ignore }),
            };
            html! {
                <ProgramModuleComponent<Controller, T, ProgramModuleComponentImplTypeListOnly> with module/>
            }
        });
        html! {
            <div onmousemove=mouse_move class="dotevery_editor">
                // {"DotEvery.Editor"}
                // {self.logic_data.id}
                <div class="editor_window">
                    <div ref=self.trash_area_ref.clone() class="program_module_palette">
                        {for palette}
                    </div>
                    <ProgramModuleComponent<Controller, T, ProgramModuleComponentImplTypeDefault> with module/>
                </div>
                {dragging}
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
        if let Some(element) = self.trash_area_ref.cast::<Element>() {
            let rect = element.get_bounding_client_rect();
            let offset = get_page_offset();
            self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetTrashArea {
                x: rect.x() + offset.0,
                y: rect.y() + offset.1,
                w: rect.width(),
                h: rect.height(),
            });
        }
    }
}
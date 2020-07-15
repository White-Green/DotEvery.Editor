use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage};
use crate::components::program_module::ProgramModuleComponent;
use crate::components::program_module_list::{ProgramModuleListComponent, ProgramModuleListProperties};
use crate::logic::dotevery_editor::DotEveryEditor;
use crate::util;

#[derive(Clone, Properties)]
pub(crate) struct DotEveryEditorProperties {
    pub(crate) dotevery_editor: DotEveryEditor,
}

pub(crate) struct DotEveryEditorComponent {
    link: ComponentLink<Self>,
    props: DotEveryEditorProperties,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent>>,
}

pub(crate) enum DotEveryEditorMessage {
    Ignore,
    DragModuleAgentMessage(DragModuleAgentInputMessage),
}

impl Component for DotEveryEditorComponent {
    type Message = DotEveryEditorMessage;
    type Properties = DotEveryEditorProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Self::Message::Ignore);
        Self {
            link,
            props,
            drag_module_agent_bridge: DragModuleAgent::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::DragModuleAgentMessage(msg) => {
                self.drag_module_agent_bridge.send(msg);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let list = self.props.dotevery_editor.list.clone();
        let list = ProgramModuleListProperties {
            program_module_list: list,
            rect_changed_callback: None,
        };
        html! {
            <div class="dotevery_editor">
                {"DotEvery.Editor"}
                <ProgramModuleListComponent with list/>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let window = web_sys::window().unwrap();
            let callback = self.link.callback(|m| m);
            let closure = Closure::wrap(Box::new(move |e: MouseEvent| {
                callback.emit(Self::Message::DragModuleAgentMessage(DragModuleAgentInputMessage::UpdateMousePosition { x: e.page_x(), y: e.page_y() }));
                if e.buttons() != 1 {
                    callback.emit(Self::Message::DragModuleAgentMessage(DragModuleAgentInputMessage::EndDrag));
                }
            }) as Box<dyn FnMut(_)>);
            if let Err(err) = window.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()) {
                clog!("add mousemove event failed",err);
            }
            closure.forget();
        }
    }
}
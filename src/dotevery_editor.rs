use crate::util;
use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::program_module::program_module_list::{ProgramModuleListProperties, ProgramModuleList};
use crate::program_module::program_module::{ProgramModuleProperties, ProgramModule};

pub(crate) struct DotEveryEditor {
    link: ComponentLink<Self>,
    props: DotEveryEditorProperties,
}

pub enum DotEveryEditorMessage {}

#[derive(Clone, Properties)]
pub struct DotEveryEditorProperties {
    pub(crate) list: ProgramModuleListProperties,
}

impl Component for DotEveryEditor {
    type Message = DotEveryEditorMessage;
    type Properties = DotEveryEditorProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.props = _props;
        true
    }

    fn view(&self) -> Html {
        let list = self.props.list.clone();
        html! {
            <div class="dotevery_editor">
                {"DotEvery.Editor"}
                <ProgramModuleList with list/>
            </div>
        }
    }
}
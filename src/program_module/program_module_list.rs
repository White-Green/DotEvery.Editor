use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::program_module::program_module::{ProgramModuleProperties, ProgramModule};

pub struct ProgramModuleList {
    link: ComponentLink<Self>,
    props: ProgramModuleListProperties,
}

pub enum ProgramModuleListMessage {}

#[derive(Clone, Properties)]
pub struct ProgramModuleListProperties {
    pub(crate) children: Vec<ProgramModuleProperties>,
}

impl Component for ProgramModuleList {
    type Message = ProgramModuleListMessage;
    type Properties = ProgramModuleListProperties;

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
        let options = self.props.children.iter().map(|p| {
            let p = p.clone();
            html! {
                <ProgramModule with p/>
            }
        });
        html! {
            <div class="program_module_list">
                {for options}
            </div>
        }
    }
}
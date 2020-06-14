use yew::{ComponentLink, Component, Html};
use yew::prelude::*;
use crate::program_module::program_module::{ProgramModuleProperties, ProgramModule};

pub struct ProgramModuleList {
    link: ComponentLink<Self>,
    props: ProgramModuleListProperties,
}

pub enum ProgramModuleListMessage {
    Ignore,
    UpdateChildRect,
}

#[derive(Clone, Properties)]
pub struct ProgramModuleListProperties {
    children: Vec<ProgramModuleProperties>,
    rect_changed_callback: Option<Callback<()>>,
}

impl ProgramModuleListProperties {
    pub fn new(children: Vec<ProgramModuleProperties>) -> Self {
        Self {
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
        let rect_changed_callback = link.callback(|_| Self::Message::UpdateChildRect);
        for module in &mut props.children {
            module.rect_changed_callback = Some(rect_changed_callback.clone());
        }
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::UpdateChildRect => {
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
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
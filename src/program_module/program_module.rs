use yew::{ComponentLink, Component, Html};
use yew::prelude::*;

pub(crate) struct ProgramModule {
    link: ComponentLink<Self>,
    props: ProgramModuleProperties,
}

pub enum ProgramModuleMessage {}

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
    pub(crate) options: Vec<ProgramModuleOption>,
    pub(crate) child: ProgramModuleChildItems,
}

impl Component for ProgramModule {
    type Message = ProgramModuleMessage;
    type Properties = ProgramModuleProperties;

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
        let options = self.props.options.iter().map(
            |o| match o {
                ProgramModuleOption::StringSign(s) => html! {<span class="program_module_option program_module_option_string_sign">{s}</span>},
                ProgramModuleOption::StringInput(s) => html! {<input class="program_module_option program_module_option_string_input" value={s}/>},
                ProgramModuleOption::ProgramModule(p) => match p {
                    Some(p) => {
                        let p = p.clone();
                        html! {<div class="program_module_option program_module_option_module"><ProgramModule with p/></div>}
                    }
                    None => html! {<div class="program_module_option program_module_option_module"></div>}
                },
            });
        html! {
            <div class="program_module">
                <div class="program_module_options">
                    {for options}
                </div>
            </div>
        }
    }
}
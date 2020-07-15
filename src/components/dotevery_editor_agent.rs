use std::collections::VecDeque;

use bimap::{BiHashMap, Overwritten};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::__rt::std::collections::{HashMap, HashSet};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::util::Rect;

pub(crate) struct DotEveryEditorAgent {
    link: AgentLink<Self>,
}

pub(crate) enum DotEveryEditorAgentMessage {}

#[derive(Serialize, Deserialize)]
pub(crate) enum DotEveryEditorAgentInputMessage {}

#[derive(Serialize, Deserialize)]
pub(crate) enum DotEveryEditorAgentOutputMessage {}

impl Agent for DotEveryEditorAgent {
    type Reach = Context;
    type Message = DotEveryEditorAgentMessage;
    type Input = DotEveryEditorAgentInputMessage;
    type Output = DotEveryEditorAgentOutputMessage;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {}
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) { match msg {} }
}
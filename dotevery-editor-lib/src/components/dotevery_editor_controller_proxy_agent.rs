use yew::{Bridge, Bridged};
use yew::agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::dotevery_editor_agent::{DotEveryEditorAgent, DotEveryEditorAgentInputMessage, DotEveryEditorAgentOutputMessage};
use crate::logic::dotevery_editor_controller::DotEveryEditorController;

pub struct DotEveryEditorControllerProxyAgent<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    link: AgentLink<Self>,
    logic: Box<dyn Bridge<DotEveryEditorAgent<Controller, Type>>>,
}

pub enum ProxyMessage<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq {
    Ignore,
    Response(HandlerId, Controller::Output),
}

impl<Controller, Type> Agent for DotEveryEditorControllerProxyAgent<Controller, Type>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq
{
    type Reach = Context<Self>;
    type Message = ProxyMessage<Controller, Type>;
    type Input = Controller::Input;
    type Output = Controller::Output;

    fn create(link: AgentLink<Self>) -> Self {
        let mut bridge = DotEveryEditorAgent::bridge(link.callback(
            |msg|
                match msg {
                    DotEveryEditorAgentOutputMessage::ResponseFromController(id, msg) => ProxyMessage::Response(id, msg),
                    _ => ProxyMessage::Ignore,
                }));
        bridge.send(DotEveryEditorAgentInputMessage::SetMeControllerProxy);
        Self {
            link,
            logic: bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            ProxyMessage::Ignore => {}
            ProxyMessage::Response(id, msg) => {
                self.link.respond(id, msg);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        self.logic.send(DotEveryEditorAgentInputMessage::MsgToController(id, msg));
    }
}
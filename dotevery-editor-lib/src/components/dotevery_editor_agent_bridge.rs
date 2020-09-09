use yew::{Bridge, Callback};
use yew::agent::{Agent, HandlerId};

use crate::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};

pub struct DotEveryEditorAgentBridge<Controller: DotEveryEditorController<T>, T: 'static + Clone + PartialEq = ()> {
    update: Callback<DotEveryEditorCommand>,
    response: Callback<(HandlerId, Controller::Output)>,
}

impl<C: DotEveryEditorController<T>, T: 'static + Clone + PartialEq> DotEveryEditorAgentBridge<C, T> {
    pub(crate) fn create(update: Callback<DotEveryEditorCommand>, response: Callback<(HandlerId, C::Output)>) -> Self {
        Self { update, response }
    }

    pub fn notify_update(&self, msg: DotEveryEditorCommand) {
        self.update.emit(msg);
    }

    pub fn respond(&self, id: HandlerId, msg: C::Output) {
        self.response.emit((id, msg));
    }
}
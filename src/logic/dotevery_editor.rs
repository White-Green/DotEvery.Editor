use uuid::Uuid;
use yew::prelude::Properties;

use crate::logic::program_module::ProgramModule;
use crate::logic::program_module_list::ProgramModuleList;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum DotEveryEditorErrorMessage {
    IndexOutOfRange,
    NotFound,
    ModuleToGetMustBeProgramModule,
    ErrorInGetModule(Box<DotEveryEditorErrorMessage>),
    ErrorInAddModule(Box<DotEveryEditorErrorMessage>),
    OptionDoesNotExpectProgramModule,
    CanNotReplace,
}

pub(crate) type DotEveryEditorResult<T> = Result<T, DotEveryEditorErrorMessage>;

#[derive(Clone)]
pub(crate) struct DotEveryEditor {
    pub(crate) list: ProgramModuleList,
}

impl DotEveryEditor {
    pub fn new(list: ProgramModuleList) -> Self {
        Self {
            list,
        }
    }

    pub fn add(&mut self, target: Uuid, index: usize, module: ProgramModule) -> DotEveryEditorResult<()> {
        self.list.add(target, index, module)
    }

    pub fn copy(&mut self, dest: Uuid, dest_index: usize, src: Uuid) -> DotEveryEditorResult<()> {
        let module = match self.get_module(src) {
            Ok(module) => module,
            Err(msg) => return Err(DotEveryEditorErrorMessage::ErrorInGetModule(Box::new(msg)))
        };
        match self.add(dest, dest_index, module) {
            Ok(_) => Ok(()),
            Err(msg) => Err(DotEveryEditorErrorMessage::ErrorInAddModule(Box::new(msg))),
        }
    }

    fn get_module(&mut self, id: Uuid) -> DotEveryEditorResult<ProgramModule> {
        self.list.get_module(id)
    }

    pub fn remove(&mut self, id: Uuid) -> DotEveryEditorResult<()> {
        if self.list.id == id {
            self.list = ProgramModuleList::new(Vec::new());
            Ok(())
        } else {
            self.list.remove(id)
        }
    }
}
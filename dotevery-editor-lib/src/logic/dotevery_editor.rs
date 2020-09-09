use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::logic::program_module::ProgramModule;

// use crate::logic::program_module_list::ProgramModuleList;

#[derive(PartialEq, Debug, Clone)]
pub enum DotEveryEditorErrorMessage {
    IndexOutOfRange,
    NotFound,
    ErrorInGetModule(Box<DotEveryEditorErrorMessage>),
    ErrorInAddModule(Box<DotEveryEditorErrorMessage>),
    OptionDoesNotExpectProgramModule,
    ChildDoesNotExpectProgramModule,
    CanNotReplace,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum DotEveryEditorOperationIndex {
    OptionAbsolute(usize),
    OptionInputFiltered(usize),
    OptionProgramModuleFiltered(usize),
    Child(usize),
}

pub(crate) type DotEveryEditorResult<T> = Result<T, DotEveryEditorErrorMessage>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DotEveryEditor<Type: 'static + Clone + PartialEq = ()> {
    pub(crate) id: Uuid,
    pub list: Vec<ProgramModule<Type>>,
}

impl<T: 'static + Clone + PartialEq> DotEveryEditor<T> {
    pub fn new(mut list: Vec<ProgramModule<T>>) -> Self {
        let id = Uuid::new_v4();
        list.iter_mut().for_each(|module| module.parent = Some(id));
        Self {
            id,
            list,
        }
    }

    pub fn add(&mut self, target: Uuid, index: DotEveryEditorOperationIndex, module: &ProgramModule<T>) -> DotEveryEditorResult<()> {
        if target.is_nil() || target == self.id {
            if let DotEveryEditorOperationIndex::Child(index) = index {
                if index <= self.list.len() {
                    self.list.insert(index, module.clone());
                    Ok(())
                } else {
                    Err(DotEveryEditorErrorMessage::IndexOutOfRange)
                }
            } else {
                Err(DotEveryEditorErrorMessage::IndexOutOfRange)
            }
        } else {
            self.list.iter_mut().find_map(|m| match m.add(target, index, module) {
                Err(DotEveryEditorErrorMessage::NotFound) => None,
                result => Some(result)
            }).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
        }
    }

    pub fn copy(&mut self, src: Uuid, dest: Uuid, dest_index: DotEveryEditorOperationIndex) -> DotEveryEditorResult<()> {
        let module = match self.get_module(src) {
            Ok(module) => module,
            Err(msg) => return Err(DotEveryEditorErrorMessage::ErrorInGetModule(Box::new(msg)))
        };
        match self.add(dest, dest_index, &module.deep_clone()) {
            Ok(_) => Ok(()),
            Err(msg) => Err(DotEveryEditorErrorMessage::ErrorInAddModule(Box::new(msg))),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn get_module(&self, id: Uuid) -> DotEveryEditorResult<&ProgramModule<T>> {
        self.list.iter().find_map(move |module| match module.get_module(id) {
            Err(DotEveryEditorErrorMessage::NotFound) => None,
            result => Some(result),
        }).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
    }

    pub fn get_module_mut(&mut self, id: Uuid) -> DotEveryEditorResult<&mut ProgramModule<T>> {
        self.list.iter_mut().find_map(move |module| match module.get_module_mut(id) {
            Err(DotEveryEditorErrorMessage::NotFound) => None,
            result => Some(result),
        }).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
    }

    pub fn get_modules_by_type(&self, type_data: &T) -> Vec<&ProgramModule<T>> {
        self.list.iter().map(|module| {
            module.get_modules_by_type(type_data)
        }).flatten().collect()
    }

    pub fn remove(&mut self, id: Uuid) -> DotEveryEditorResult<()> {
        if let Some(i) = self.list
            .iter()
            .position(move |module| module.id == id) {
            self.list.remove(i);
            Ok(())
        } else if let Some(result) = self.list
            .iter_mut()
            .find_map(|module| match module.remove(id) {
                DotEveryEditorResult::Err(DotEveryEditorErrorMessage::NotFound) => None,
                result => Some(result),
            }) {
            result
        } else {
            Err(DotEveryEditorErrorMessage::NotFound)
        }
    }

    pub fn set_root_children(&mut self, mut children: Vec<ProgramModule<T>>) -> DotEveryEditorResult<()> {
        children.iter_mut().for_each(|module| module.parent = Some(self.id));
        self.list = children;
        Ok(())
    }
}
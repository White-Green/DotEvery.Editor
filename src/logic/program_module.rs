use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::logic::dotevery_editor::{DotEveryEditorErrorMessage, DotEveryEditorResult};
use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Isomorphism;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgramModuleOption {
    StringSign(String),
    StringInput(String),
    ProgramModule(Option<ProgramModule>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgramModuleChildItems {
    None,
    Block(ProgramModuleList),
    MultiBlock(Vec<ProgramModuleList>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramModule {
    pub(crate) id: Uuid,
    pub(crate) parent: Option<Uuid>,
    pub(crate) options: Vec<ProgramModuleOption>,
    pub(crate) child: ProgramModuleChildItems,
    // pub(crate) rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
}

impl ProgramModule {
    pub fn new(mut options: Vec<ProgramModuleOption>, mut child: ProgramModuleChildItems) -> Self {
        let id = Uuid::new_v4();
        for option in &mut options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.parent = Some(id);
            }
        }
        match &mut child {
            ProgramModuleChildItems::None => {}
            ProgramModuleChildItems::Block(list) => list.parent = Some(id),
            ProgramModuleChildItems::MultiBlock(lists) =>
                for list in lists { list.parent = Some(id); },
        }
        Self {
            id,
            parent: None,
            options,
            child,
            // rect_changed_callback: None,
        }
    }

    pub fn add(&mut self, target: Uuid, index: usize, module: ProgramModule) -> DotEveryEditorResult<()> {
        if self.id == target {
            if let Some(m) = self.options.get_mut(index) {
                if let ProgramModuleOption::ProgramModule(m) = m {
                    if let Some(_) = m {
                        Err(DotEveryEditorErrorMessage::CanNotReplace)
                    } else {
                        let mut module = module;
                        module.parent = Some(self.id);
                        *m = Some(module);
                        Ok(())
                    }
                } else {
                    Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule)
                }
            } else {
                Err(DotEveryEditorErrorMessage::IndexOutOfRange)
            }
        } else {
            for option in &mut self.options {
                if let ProgramModuleOption::ProgramModule(Some(m)) = option {
                    match m.add(target, index, module.clone()) {
                        Ok(_) => return Ok(()),
                        Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                        _ => {}
                    }
                }
            }
            match &mut self.child {
                ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
                ProgramModuleChildItems::Block(list) => list.add(target, index, module),
                ProgramModuleChildItems::MultiBlock(lists) => {
                    for list in lists {
                        match list.add(target, index, module.clone()) {
                            Ok(_) => return Ok(()),
                            Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                            _ => {}
                        }
                    }
                    Err(DotEveryEditorErrorMessage::NotFound)
                }
            }
        }
    }

    pub fn get_module(&self, id: Uuid) -> DotEveryEditorResult<ProgramModule> {
        if self.id == id {
            Ok(self.clone())
        } else {
            for option in &self.options {
                if let ProgramModuleOption::ProgramModule(Some(m)) = option {
                    match m.get_module(id) {
                        Ok(module) => return Ok(module),
                        Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                        _ => {}
                    }
                }
            }
            match &self.child {
                ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
                ProgramModuleChildItems::Block(list) => list.get_module(id),
                ProgramModuleChildItems::MultiBlock(lists) => {
                    for list in lists {
                        match list.get_module(id) {
                            Ok(module) => return Ok(module),
                            Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                            _ => {}
                        }
                    }
                    Err(DotEveryEditorErrorMessage::NotFound)
                }
            }
        }
    }

    pub fn remove(&mut self, id: Uuid) -> DotEveryEditorResult<()> {
        for option in &mut self.options {
            if let ProgramModuleOption::ProgramModule(m) = option {
                let mut to_remove_this = false;
                if let Some(m) = m {
                    to_remove_this = m.id == id;
                }
                if to_remove_this {
                    *m = None;
                    return Ok(());
                }

                if let Some(m) = m {
                    match m.remove(id) {
                        Ok(_) => return Ok(()),
                        Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                        _ => {}
                    }
                }
            }
        }
        match &mut self.child {
            ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
            ProgramModuleChildItems::Block(list) => {
                if list.id == id {
                    list.children.clear();
                    Ok(())
                } else {
                    list.remove(id)
                }
            }
            ProgramModuleChildItems::MultiBlock(lists) => {
                for i in 0..lists.len() {
                    let list = &mut lists[i];
                    if list.id == id {
                        lists.remove(i);
                        return Ok(());
                    }
                    match list.remove(id) {
                        Ok(_) => return Ok(()),
                        Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                        _ => {}
                    }
                }
                Err(DotEveryEditorErrorMessage::NotFound)
            }
        }
    }

    pub fn deep_clone(&self) -> Self {
        let options = self.options.iter().map(|option| {
            match option {
                ProgramModuleOption::ProgramModule(Some(module)) => ProgramModuleOption::ProgramModule(Some(module.deep_clone())),
                other => other.clone()
            }
        }).collect();
        let child = match &self.child {
            ProgramModuleChildItems::None => ProgramModuleChildItems::None,
            ProgramModuleChildItems::Block(list) => ProgramModuleChildItems::Block(list.deep_clone()),
            ProgramModuleChildItems::MultiBlock(lists) => {
                let new_lists = lists.iter().map(ProgramModuleList::deep_clone).collect();
                ProgramModuleChildItems::MultiBlock(new_lists)
            }
        };

        let mut new_module = Self::new(options, child);

        for option in &mut new_module.options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.parent = Some(new_module.id);
            }
        }
        match &mut new_module.child {
            ProgramModuleChildItems::None => {}
            ProgramModuleChildItems::Block(list) => list.parent = Some(new_module.id),
            ProgramModuleChildItems::MultiBlock(lists) => {
                for list in lists {
                    list.parent = Some(new_module.id);
                }
            }
        }
        new_module
    }
}

impl Isomorphism for ProgramModule {
    fn isomorphisms(&self, other: &Self) -> bool {
        if self.options.len() != other.options.len() { return false; }
        let options_isomorphisms = self.options
            .iter()
            .zip(&other.options)
            .all(|(a, b)| {
                match a {
                    ProgramModuleOption::StringSign(s) => {
                        if let ProgramModuleOption::StringSign(other) = b {
                            s == other
                        } else {
                            false
                        }
                    }
                    ProgramModuleOption::StringInput(_) => {
                        if let ProgramModuleOption::StringInput(_) = b {
                            true
                        } else {
                            false
                        }
                    }
                    ProgramModuleOption::ProgramModule(module) => {
                        if let ProgramModuleOption::ProgramModule(other) = b {
                            if let Some(module) = module {
                                if let Some(other) = other {
                                    module.isomorphisms(other)
                                } else {
                                    false
                                }
                            } else {
                                other == &None
                            }
                        } else {
                            false
                        }
                    }
                }
            });
        if !options_isomorphisms { return false; }
        match &self.child {
            ProgramModuleChildItems::None => {
                other.child == ProgramModuleChildItems::None
            }
            ProgramModuleChildItems::Block(list) => {
                if let ProgramModuleChildItems::Block(other) = &other.child {
                    list.isomorphisms(other)
                } else {
                    false
                }
            }
            ProgramModuleChildItems::MultiBlock(lists) => {
                if let ProgramModuleChildItems::MultiBlock(other) = &other.child {
                    if lists.len() == other.len() {
                        lists.iter().zip(other).all(|(a, b)| a.isomorphisms(b))
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }
}
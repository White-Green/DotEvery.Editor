use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::logic::dotevery_editor::{DotEveryEditorErrorMessage, DotEveryEditorOperationIndex, DotEveryEditorResult};
// use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Isomorphism;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgramModuleOption<T: 'static + Clone + PartialEq> {
    StringSign(String),
    StringInput(String),
    ProgramModule(Option<ProgramModule<T>>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgramModuleChildItems<T: 'static + Clone + PartialEq> {
    None,
    BlockVertical(Vec<ProgramModule<T>>),
    BlockHorizontal(Vec<ProgramModule<T>>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramModule<Type: 'static + Clone + PartialEq = ()> {
    pub(crate) id: Uuid,
    pub(crate) parent: Option<Uuid>,
    pub options: Vec<ProgramModuleOption<Type>>,
    pub child: ProgramModuleChildItems<Type>,
    pub type_data: Type,
    // pub(crate) rect_changed_callback: Option<Callback<(Uuid, Rect)>>,
}

impl<T: 'static + Clone + PartialEq + Default> ProgramModule<T> {
    pub fn new_default(options: Vec<ProgramModuleOption<T>>, child: ProgramModuleChildItems<T>) -> Self {
        Self::new(options, child, Default::default())
    }

    pub(crate) fn new_default_with_id(id: Uuid, options: Vec<ProgramModuleOption<T>>, child: ProgramModuleChildItems<T>) -> Self {
        Self::new_with_id(id, options, child, Default::default())
    }
}

impl<T: 'static + Clone + PartialEq> ProgramModule<T> {
    pub fn new(options: Vec<ProgramModuleOption<T>>, child: ProgramModuleChildItems<T>, type_data: T) -> Self {
        let id = Uuid::new_v4();
        Self::new_with_id(id, options, child, type_data)
    }

    pub(crate) fn new_with_id(id: Uuid, mut options: Vec<ProgramModuleOption<T>>, mut child: ProgramModuleChildItems<T>, type_data: T) -> Self {
        for option in &mut options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.parent = Some(id);
            }
        }
        match &mut child {
            ProgramModuleChildItems::None => {}
            ProgramModuleChildItems::BlockVertical(list) =>
                Self::set_parent_to_list(id)(list),
            ProgramModuleChildItems::BlockHorizontal(list) =>
                Self::set_parent_to_list(id)(list),
        }
        Self {
            id,
            parent: None,
            options,
            child,
            // rect_changed_callback: None,
            type_data,
        }
    }

    pub(crate) fn isomorphic_transform<U: 'static + Clone + PartialEq + Default>(&self) -> ProgramModule<U> {
        ProgramModule {
            id: self.id.clone(),
            parent: self.parent.clone(),
            options: self.options.iter().map(|option| match option {
                ProgramModuleOption::ProgramModule(Some(module)) => ProgramModuleOption::ProgramModule(Some(module.isomorphic_transform())),
                ProgramModuleOption::ProgramModule(None) => ProgramModuleOption::ProgramModule(None),
                ProgramModuleOption::StringSign(s) => ProgramModuleOption::StringSign(s.clone()),
                ProgramModuleOption::StringInput(s) => ProgramModuleOption::StringInput(s.clone()),
            }).collect(),
            child: match &self.child {
                ProgramModuleChildItems::BlockVertical(list) => ProgramModuleChildItems::BlockVertical(list.iter().map(Self::isomorphic_transform).collect()),
                ProgramModuleChildItems::BlockHorizontal(list) => ProgramModuleChildItems::BlockHorizontal(list.iter().map(Self::isomorphic_transform).collect()),
                ProgramModuleChildItems::None => ProgramModuleChildItems::None,
            },
            type_data: Default::default(),
        }
    }

    pub fn add(&mut self, target: Uuid, index: DotEveryEditorOperationIndex, module: &ProgramModule<T>) -> DotEveryEditorResult<()> {
        if self.id == target {
            let mut module = module.clone();
            module.parent = Some(self.id);
            match index {
                DotEveryEditorOperationIndex::OptionAbsolute(index) => {
                    if let Some(m) = self.options.get_mut(index) {
                        if let ProgramModuleOption::ProgramModule(m) = m {
                            if let Some(_) = m {
                                Err(DotEveryEditorErrorMessage::CanNotReplace)
                            } else {
                                *m = Some(module);
                                Ok(())
                            }
                        } else {
                            Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule)
                        }
                    } else {
                        Err(DotEveryEditorErrorMessage::IndexOutOfRange)
                    }
                }
                DotEveryEditorOperationIndex::OptionInputFiltered(mut index) => {
                    for option in &mut self.options {
                        if index == 0 {
                            match option {
                                ProgramModuleOption::StringSign(_) => {}
                                ProgramModuleOption::StringInput(_) => { return Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule); }
                                ProgramModuleOption::ProgramModule(field) => {
                                    return if let Some(_) = field {
                                        Err(DotEveryEditorErrorMessage::CanNotReplace)
                                    } else {
                                        *field = Some(module);
                                        Ok(())
                                    };
                                }
                            }
                        } else {
                            match option {
                                ProgramModuleOption::StringSign(_) => {}
                                ProgramModuleOption::StringInput(_) => { index -= 1; }
                                ProgramModuleOption::ProgramModule(_) => { index -= 1; }
                            }
                        }
                    }
                    Err(DotEveryEditorErrorMessage::IndexOutOfRange)
                }
                DotEveryEditorOperationIndex::OptionProgramModuleFiltered(mut index) => {
                    for option in &mut self.options {
                        if index == 0 {
                            if let ProgramModuleOption::ProgramModule(field) = option {
                                return if let Some(_) = field {
                                    Err(DotEveryEditorErrorMessage::CanNotReplace)
                                } else {
                                    *field = Some(module);
                                    Ok(())
                                };
                            }
                        } else {
                            match option {
                                ProgramModuleOption::ProgramModule(_) => { index -= 1; }
                                _ => {}
                            }
                        }
                    }
                    Err(DotEveryEditorErrorMessage::IndexOutOfRange)
                }
                DotEveryEditorOperationIndex::Child(index) => {
                    fn add_inner<T: Clone + PartialEq>(list: &mut Vec<ProgramModule<T>>, index: usize, module: ProgramModule<T>) -> DotEveryEditorResult<()> {
                        if list.len() >= index {
                            list.insert(index, module);
                            Ok(())
                        } else {
                            Err(DotEveryEditorErrorMessage::IndexOutOfRange)
                        }
                    }
                    match &mut self.child {
                        ProgramModuleChildItems::None => { Err(DotEveryEditorErrorMessage::ChildDoesNotExpectProgramModule) }
                        ProgramModuleChildItems::BlockVertical(list) => add_inner(list, index, module),
                        ProgramModuleChildItems::BlockHorizontal(list) => add_inner(list, index, module),
                    }
                }
            }
        } else {
            for option in &mut self.options {
                if let ProgramModuleOption::ProgramModule(Some(m)) = option {
                    match m.add(target, index, module) {
                        Ok(_) => return Ok(()),
                        Err(msg)if msg != DotEveryEditorErrorMessage::NotFound => return Err(msg),
                        _ => {}
                    }
                }
            }
            match &mut self.child {
                ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
                ProgramModuleChildItems::BlockVertical(list) =>
                    Self::add_to_list(target, index, module)(list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound)),
                ProgramModuleChildItems::BlockHorizontal(list) =>
                    Self::add_to_list(target, index, module)(list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound)),
            }
        }
    }

    fn add_to_list<'a>(target: Uuid, index: DotEveryEditorOperationIndex, module: &'a ProgramModule<T>) -> impl 'a + Fn(&mut Vec<ProgramModule<T>>) -> Option<DotEveryEditorResult<()>> {
        move |list| list.iter_mut().find_map(|m| match m.add(target, index, module) {
            Err(DotEveryEditorErrorMessage::NotFound) => None,
            result => Some(result),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn get_module(&self, id: Uuid) -> DotEveryEditorResult<&ProgramModule<T>> {
        if self.id == id {
            Ok(self)
        } else {
            self.options
                .iter()
                .find_map(|option|
                    match option {
                        ProgramModuleOption::ProgramModule(Some(module)) =>
                            match module.get_module(id) {
                                Err(DotEveryEditorErrorMessage::NotFound) => None,
                                result => Some(result),
                            }
                        _ => None
                    }
                )
                .unwrap_or_else(
                    || {
                        match &self.child {
                            ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
                            ProgramModuleChildItems::BlockVertical(list) =>
                                Self::get_module_from_list(id, list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound)),
                            ProgramModuleChildItems::BlockHorizontal(list) => {
                                Self::get_module_from_list(id, list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
                            }
                        }
                    }
                )
        }
    }

    pub fn get_module_mut(&mut self, id: Uuid) -> DotEveryEditorResult<&mut ProgramModule<T>> {
        if self.id == id {
            Ok(self)
        } else {
            let result = self.options
                .iter_mut()
                .find_map(|option|
                    match option {
                        ProgramModuleOption::ProgramModule(Some(module)) =>
                            match module.get_module_mut(id) {
                                Err(DotEveryEditorErrorMessage::NotFound) => None,
                                result => Some(result),
                            }
                        _ => None
                    }
                );
            if let Some(result) = result {
                result
            } else {
                match &mut self.child {
                    ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
                    ProgramModuleChildItems::BlockVertical(list) =>
                        Self::get_module_mut_from_list(id, list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound)),
                    ProgramModuleChildItems::BlockHorizontal(list) => {
                        Self::get_module_mut_from_list(id, list).unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
                    }
                }
            }
        }
    }

    pub fn get_modules_by_type(&self, type_data: &T) -> Vec<&ProgramModule<T>> {
        let mut result = if &self.type_data == type_data {
            vec![self]
        } else {
            Vec::new()
        };
        let mut option = self.options.iter().map(|option| {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.get_modules_by_type(type_data)
            } else {
                Vec::new()
            }
        }).flatten().collect();
        let mut child = match &self.child {
            ProgramModuleChildItems::None => { Vec::new() }
            ProgramModuleChildItems::BlockVertical(list) | ProgramModuleChildItems::BlockHorizontal(list) => {
                list.iter().map(|module| {
                    module.get_modules_by_type(type_data)
                }).flatten().collect()
            }
        };
        result.append(&mut option);
        result.append(&mut child);
        result
    }

    fn get_module_from_list(id: Uuid, list: &Vec<ProgramModule<T>>) -> Option<DotEveryEditorResult<&ProgramModule<T>>> {
        list.iter()
            .find_map(
                |module| {
                    match module.get_module(id) {
                        Err(DotEveryEditorErrorMessage::NotFound) => None,
                        result => Some(result),
                    }
                })
    }

    fn get_module_mut_from_list(id: Uuid, list: &mut Vec<ProgramModule<T>>) -> Option<DotEveryEditorResult<&mut ProgramModule<T>>> {
        list.iter_mut()
            .find_map(
                |module| {
                    match module.get_module_mut(id) {
                        Err(DotEveryEditorErrorMessage::NotFound) => None,
                        result => Some(result),
                    }
                })
    }

    pub fn remove(&mut self, id: Uuid) -> DotEveryEditorResult<()> {
        let index = self.options.iter().position(|option| {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.id == id
            } else {
                false
            }
        });
        if let Some(index) = index {
            self.options[index] = ProgramModuleOption::ProgramModule(None);
            return Ok(());
        }
        let result = self.options.iter_mut().find_map(|option|
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                match module.remove(id) {
                    Err(DotEveryEditorErrorMessage::NotFound) => None,
                    result => Some(result),
                }
            } else {
                None
            });
        if let Some(result) = result {
            debug_assert_ne!(result, Err(DotEveryEditorErrorMessage::NotFound));
            return result;
        }

        match &mut self.child {
            ProgramModuleChildItems::None => Err(DotEveryEditorErrorMessage::NotFound),
            ProgramModuleChildItems::BlockVertical(list) => {
                Self::remove_module_from_list(id)(list)
                    .unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
            }
            ProgramModuleChildItems::BlockHorizontal(list) => {
                Self::remove_module_from_list(id)(list)
                    .unwrap_or(Err(DotEveryEditorErrorMessage::NotFound))
            }
        }
    }

    fn remove_module_from_list(id: Uuid) -> impl Fn(&mut Vec<ProgramModule<T>>) -> Option<DotEveryEditorResult<()>> {
        move |list| {
            if let Some(index) = list.iter().position(|module| module.id == id) {
                list.remove(index);
                Some(Ok(()))
            } else {
                list.iter_mut().find_map(|module| match module.remove(id) {
                    Err(DotEveryEditorErrorMessage::NotFound) => None,
                    result => Some(result),
                })
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
            ProgramModuleChildItems::BlockVertical(list) =>
                ProgramModuleChildItems::BlockVertical(Self::deep_clone_list(list)),
            ProgramModuleChildItems::BlockHorizontal(list) =>
                ProgramModuleChildItems::BlockHorizontal(Self::deep_clone_list(list)),
        };

        let mut new_module = Self::new(options, child, self.type_data.clone());

        let id = new_module.id;
        for option in &mut new_module.options {
            if let ProgramModuleOption::ProgramModule(Some(module)) = option {
                module.parent = Some(id);
            }
        }
        match &mut new_module.child {
            ProgramModuleChildItems::None => {}
            ProgramModuleChildItems::BlockVertical(list) =>
                Self::set_parent_to_list(id)(list),
            ProgramModuleChildItems::BlockHorizontal(list) =>
                Self::set_parent_to_list(id)(list)
        }
        new_module
    }

    fn set_parent_to_list(id: Uuid) -> impl Fn(&mut Vec<ProgramModule<T>>) {
        move |list| list.iter_mut().for_each(|module| module.parent = Some(id))
    }

    fn deep_clone_list(list: &Vec<ProgramModule<T>>) -> Vec<ProgramModule<T>> {
        list.iter().map(ProgramModule::deep_clone).collect()
    }

    fn list_isomorphisms(input: (&Vec<ProgramModule<T>>, &Vec<ProgramModule<T>>)) -> bool {
        let (a, b) = input;
        a.len() == b.len() && b.iter().zip(b).all(|(a, b)| a.isomorphisms(b))
    }
}

impl<T: 'static + Clone + PartialEq> Isomorphism for ProgramModule<T> {
    fn isomorphisms(&self, other: &Self) -> bool {
        if self.options.len() != other.options.len() { return false; }
        if self.type_data != other.type_data { return false; }
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
            ProgramModuleChildItems::BlockVertical(list) => {
                if let ProgramModuleChildItems::BlockVertical(other) = &other.child {
                    Self::list_isomorphisms((list, other))
                } else {
                    false
                }
            }
            ProgramModuleChildItems::BlockHorizontal(list) => {
                if let ProgramModuleChildItems::BlockHorizontal(other) = &other.child {
                    Self::list_isomorphisms((list, other))
                } else {
                    false
                }
            }
        }
    }
}
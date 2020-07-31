use uuid::Uuid;
use wasm_bindgen::__rt::core::hint::unreachable_unchecked;

use crate::logic::dotevery_editor::DotEveryEditorErrorMessage;
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Isomorphism;

#[test]
fn program_module_list_deep_clone_test() {
    let new_module = || {
        ProgramModule::new(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new(
                                            Vec::new(),
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::Block(
                                ProgramModuleList::new(
                                    (0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::MultiBlock(
                (0..10).map(|_| { ProgramModuleList::new((0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()) }).collect()
            ))
    };
    let new_module_list = || { ProgramModuleList::new((0..10).map(|_| new_module()).collect()) };

    let list = new_module_list();
    let cloned = list.deep_clone();
    assert_ne!(list, cloned);
    assert!(list.isomorphisms(&cloned));
    assert_ne!(list.id, cloned.id);
    assert_eq!(list.children.len(), cloned.children.len());
    for (module, cloned) in list.children.iter().zip(cloned.children) {
        assert!(module.isomorphisms(&cloned));
        assert_ne!(module.id, cloned.id);
    }
}

#[test]
fn program_module_list_get_module_test() {
    let new_module = || {
        ProgramModule::new(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new(
                                            Vec::new(),
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::Block(
                                ProgramModuleList::new(
                                    (0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::MultiBlock(
                (0..10).map(|_| { ProgramModuleList::new((0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()) }).collect()
            ))
    };
    let new_module_list = || { ProgramModuleList::new((0..10).map(|_| new_module()).collect()) };

    let list = new_module_list();
    let module = list.children[3].clone();
    assert_eq!(list.get_module(module.id), Ok(module));

    let list = new_module_list();
    let (id, module) = if let ProgramModuleOption::ProgramModule(Some(module)) = &list.children[3].options[2] {
        (module.id, module.clone())
    } else { unreachable!(); };
    assert_eq!(list.get_module(id), Ok(module));
}

#[test]
fn program_module_list_add_test() {
    let new_module = || {
        ProgramModule::new(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new(
                                            Vec::new(),
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::Block(
                                ProgramModuleList::new(
                                    (0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::MultiBlock(
                (0..10).map(|_| { ProgramModuleList::new((0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()) }).collect()
            ))
    };
    let new_module_list = || { ProgramModuleList::new((0..10).map(|_| new_module()).collect()) };

    let mut list = new_module_list();
    assert_eq!(list.add(Uuid::new_v4(), 0, new_module()), Err(DotEveryEditorErrorMessage::NotFound));

    let mut list = new_module_list();
    let mut expect = list.clone();
    let module = new_module();
    let mut module1 = module.clone();
    module1.parent = Some(list.id.clone());
    expect.children.insert(3, module1);
    assert_eq!(list.add(list.id, 3, module), Ok(()));
    assert_eq!(list, expect);

    let mut list = new_module_list();
    let mut expect = list.clone();
    let module = new_module();
    let mut module1 = module.clone();
    module1.parent = Some(expect.children[3].id.clone());
    expect.children[3].options[3] = ProgramModuleOption::ProgramModule(Some(module1));
    assert_eq!(list.add(list.children[3].id, 3, module), Ok(()));
    assert_eq!(list, expect);
    if let ProgramModuleOption::ProgramModule(Some(module)) = &list.children[3].options[3] {
        assert_eq!(module.parent, Some(list.children[3].id));
    } else { unreachable!(); }
}

#[test]
fn program_module_list_remove_test() {
    let new_module = || {
        ProgramModule::new(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new(
                                            Vec::new(),
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::Block(
                                ProgramModuleList::new(
                                    (0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::MultiBlock(
                (0..10).map(|_| { ProgramModuleList::new((0..10).map(|_| { ProgramModule::new(Vec::new(), ProgramModuleChildItems::None) }).collect()) }).collect()
            ))
    };
    let new_module_list = || { ProgramModuleList::new((0..10).map(|_| new_module()).collect()) };

    let mut list = new_module_list();
    assert_eq!(list.remove(Uuid::new_v4()), Err(DotEveryEditorErrorMessage::NotFound));

    let mut list = new_module_list();
    let mut expect = list.clone();
    expect.children.remove(3);
    assert_eq!(list.remove(list.children[3].id), Ok(()));
    assert_eq!(list, expect);

    let mut list = new_module_list();
    let mut expect = list.clone();
    let id = if let ProgramModuleOption::ProgramModule(Some(module)) = &list.children[3].options[2] {
        module.id
    } else { unreachable!(); };
    expect.children[3].options[2] = ProgramModuleOption::ProgramModule(None);
    assert_eq!(list.remove(id), Ok(()));
    assert_eq!(list, expect);
}
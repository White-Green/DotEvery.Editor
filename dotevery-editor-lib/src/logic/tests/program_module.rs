use uuid::Uuid;

use crate::logic::dotevery_editor::{DotEveryEditorErrorMessage, DotEveryEditorOperationIndex};
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::util::Isomorphism;

#[test]
fn program_module_deep_clone_test() {
    let module = ProgramModule::new(
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
                                    ProgramModule::new_default(
                                        Vec::new(),
                                        ProgramModuleChildItems::None)))
                        ],
                        ProgramModuleChildItems::BlockVertical(
                            (0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect())
                        , 256))),
            ProgramModuleOption::ProgramModule(None)
        ], ProgramModuleChildItems::BlockHorizontal(
            ((0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect())
        ), 128);
    let cloned = module.deep_clone();
    assert_eq!(module, module);
    assert_ne!(module, cloned);
    assert!(module.isomorphisms(&cloned));
    assert_ne!(module.id, cloned.id);
    assert_eq!(module.type_data, cloned.type_data);

    if let ProgramModuleOption::StringSign(module) = &module.options[0] {
        if let ProgramModuleOption::StringSign(cloned) = &cloned.options[0] {
            assert_eq!(module, cloned);
        } else { unreachable!(); }
    } else { unreachable!(); }

    if let ProgramModuleOption::StringInput(module) = &module.options[1] {
        if let ProgramModuleOption::StringInput(cloned) = &cloned.options[1] {
            assert_eq!(module, cloned);
        } else { unreachable!(); }
    } else { unreachable!(); }

    if let ProgramModuleOption::ProgramModule(Some(inner_module)) = &module.options[2] {
        if let ProgramModuleOption::ProgramModule(Some(inner_cloned)) = &cloned.options[2] {
            assert_ne!(inner_module.id, inner_cloned.id);
            assert_eq!(inner_module.type_data, inner_cloned.type_data);
            assert_eq!(inner_cloned.parent, Some(cloned.id));
            if let ProgramModuleChildItems::BlockVertical(module) = &inner_module.child {
                if let ProgramModuleChildItems::BlockVertical(cloned) = &inner_cloned.child {
                    assert_eq!(module.len(), cloned.len());
                    for (module, cloned) in module.iter().zip(cloned) {
                        assert!(module.isomorphisms(cloned));
                        assert_ne!(module.id, cloned.id);
                        assert_eq!(cloned.parent, Some(inner_cloned.id));
                    }
                } else { unreachable!(); }
            } else { unreachable!(); }
        } else { unreachable!(); }
    } else { unreachable!(); }

    if let ProgramModuleOption::ProgramModule(module) = &module.options[3] {
        if let ProgramModuleOption::ProgramModule(cloned) = &cloned.options[3] {
            assert_eq!(module, cloned);
        } else { unreachable!(); }
    } else { unreachable!(); }

    if let ProgramModuleChildItems::BlockHorizontal(module_list) = &module.child {
        if let ProgramModuleChildItems::BlockHorizontal(cloned_list) = &cloned.child {
            assert_eq!(module_list.len(), cloned_list.len());
            for (inner_module, inner_cloned) in module_list.iter().zip(cloned_list) {
                assert!(inner_module.isomorphisms(inner_cloned));
                assert_ne!(inner_module.id, inner_cloned.id);
                assert_eq!(inner_cloned.parent, Some(cloned.id));
            }
        } else { unreachable!(); }
    } else { unreachable!(); }
}

#[test]
fn program_module_get_module_test() {
    let module = ProgramModule::<()>::new_default(
        vec![
            ProgramModuleOption::StringSign("test".to_string()),
            ProgramModuleOption::StringInput("test2".to_string()),
            ProgramModuleOption::ProgramModule(
                Some(
                    ProgramModule::new_default(
                        vec![
                            ProgramModuleOption::StringSign("test".to_string()),
                            ProgramModuleOption::StringInput("test2".to_string()),
                            ProgramModuleOption::ProgramModule(
                                Some(
                                    ProgramModule::new_default(
                                        Vec::new(),
                                        ProgramModuleChildItems::None)))
                        ],
                        ProgramModuleChildItems::BlockVertical(
                            (
                                (0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
            ProgramModuleOption::ProgramModule(None)
        ], ProgramModuleChildItems::BlockHorizontal(
            ((0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect())
        ));
    assert_eq!(module.get_module(Uuid::new_v4()), Err(DotEveryEditorErrorMessage::NotFound));
    assert_eq!(module.get_module(module.id), Ok(&module.clone()));
    let (module_id, module_obj) = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        if let ProgramModuleChildItems::BlockVertical(list) = &module.child {
            (list[3].id, list[3].clone())
        } else { unreachable!(); }
    } else { unreachable!(); };
    assert_eq!(module.get_module(module_id), Ok(&module_obj));

    let (module_id, module_obj) = if let ProgramModuleChildItems::BlockHorizontal(lists) = &module.child {
        (lists[3].id, lists[3].clone())
    } else { unreachable!(); };
    assert_eq!(module.get_module(module_id), Ok(&module_obj));
}

#[test]
fn program_module_add_test() {
    let new_module = || {
        ProgramModule::<()>::new_default(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new_default(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new_default(
                                            Vec::new(),
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::BlockVertical(
                                (
                                    (0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::BlockHorizontal(
                ((0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect())
            ))
    };
    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionAbsolute(0), &add), Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule));

    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionInputFiltered(0), &add), Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule));

    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionAbsolute(2), &add), Err(DotEveryEditorErrorMessage::CanNotReplace));

    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionProgramModuleFiltered(0), &add), Err(DotEveryEditorErrorMessage::CanNotReplace));

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let mut module1 = add.clone();
    module1.parent = Some(module.id);
    expect.options[3] = ProgramModuleOption::ProgramModule(Some(module1));
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionAbsolute(3), &add), Ok(()));
    assert_eq!(module, expect);
    if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[3] {
        assert_eq!(option.parent, Some(module.id));
    } else { unreachable!(); }

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let mut module1 = add.clone();
    module1.parent = Some(module.id);
    expect.options[3] = ProgramModuleOption::ProgramModule(Some(module1));
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionInputFiltered(2), &add), Ok(()));
    assert_eq!(module, expect);
    if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[3] {
        assert_eq!(option.parent, Some(module.id));
    } else { unreachable!(); }

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let mut module1 = add.clone();
    module1.parent = Some(module.id);
    expect.options[3] = ProgramModuleOption::ProgramModule(Some(module1));
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::OptionProgramModuleFiltered(1), &add), Ok(()));
    assert_eq!(module, expect);
    if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[3] {
        assert_eq!(option.parent, Some(module.id));
    } else { unreachable!(); }

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        module.id
    } else { unreachable!(); };
    if let ProgramModuleOption::ProgramModule(Some(module)) = &mut expect.options[2] {
        if let ProgramModuleChildItems::BlockVertical(list) = &mut module.child {
            let mut module1 = add.clone();
            module1.parent = Some(module.id);
            list.insert(3, module1);
        } else { unreachable!(); }
    } else { unreachable!(); }
    assert_eq!(module.add(id, DotEveryEditorOperationIndex::Child(3), &add), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    if let ProgramModuleChildItems::BlockHorizontal(lists) = &mut expect.child {
        let mut module1 = add.clone();
        module1.parent = Some(module.id);
        lists.insert(3, module1);
    } else { unreachable!(); }
    assert_eq!(module.add(module.id, DotEveryEditorOperationIndex::Child(3), &add), Ok(()));
    assert_eq!(module, expect);
}

#[test]
fn program_module_remove_test() {
    let new_module: fn() -> ProgramModule = || {
        ProgramModule::new_default(
            vec![
                ProgramModuleOption::StringSign("test".to_string()),
                ProgramModuleOption::StringInput("test2".to_string()),
                ProgramModuleOption::ProgramModule(
                    Some(
                        ProgramModule::new_default(
                            vec![
                                ProgramModuleOption::StringSign("test".to_string()),
                                ProgramModuleOption::StringInput("test2".to_string()),
                                ProgramModuleOption::ProgramModule(
                                    Some(
                                        ProgramModule::new_default(
                                            vec![ProgramModuleOption::ProgramModule(
                                                Some(
                                                    ProgramModule::new_default(
                                                        Vec::new(),
                                                        ProgramModuleChildItems::None)))],
                                            ProgramModuleChildItems::None)))
                            ],
                            ProgramModuleChildItems::BlockVertical(
                                (
                                    (0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect()))))),
                ProgramModuleOption::ProgramModule(None)
            ], ProgramModuleChildItems::BlockHorizontal(
                ((0..10).map(|_| { ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None) }).collect())
            ))
    };

    let mut module = new_module();
    let id = module.id.clone();
    assert_eq!(module.remove(Uuid::new_v4()), Err(DotEveryEditorErrorMessage::NotFound));
    assert_eq!(module.remove(id), Err(DotEveryEditorErrorMessage::NotFound));

    let mut module = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[2] {
        option.id
    } else { unreachable!(); };
    let mut expect = module.clone();
    expect.options[2] = ProgramModuleOption::ProgramModule(None);
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[2] {
        if let ProgramModuleOption::ProgramModule(Some(option)) = &option.options[2] {
            option.id
        } else { unreachable!(); }
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleOption::ProgramModule(Some(option)) = &mut expect.options[2] {
        option.options[2] = ProgramModuleOption::ProgramModule(None);
    } else { unreachable!(); };
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let id = if let ProgramModuleChildItems::BlockHorizontal(lists) = &mut module.child {
        lists[3].id
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleChildItems::BlockHorizontal(lists) = &mut expect.child {
        lists.remove(3);
    } else { unreachable!(); }
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        if let ProgramModuleChildItems::BlockVertical(lists) = &module.child {
            lists[3].id
        } else { unreachable!(); }
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleOption::ProgramModule(Some(expect)) = &mut expect.options[2] {
        if let ProgramModuleChildItems::BlockVertical(lists) = &mut expect.child {
            lists.remove(3);
        } else { unreachable!(); }
    } else { unreachable!(); }
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);
}
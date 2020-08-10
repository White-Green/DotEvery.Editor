
use uuid::Uuid;

use crate::logic::dotevery_editor::{DotEveryEditorErrorMessage};
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::logic::program_module_list::ProgramModuleList;
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
        ));
    let cloned = module.deep_clone();
    assert_eq!(module, module);
    assert_ne!(module, cloned);
    assert!(module.isomorphisms(&cloned));
    assert_ne!(module.id, cloned.id);

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
            assert_eq!(inner_cloned.parent, Some(cloned.id));
            if let ProgramModuleChildItems::Block(module) = &inner_module.child {
                if let ProgramModuleChildItems::Block(cloned) = &inner_cloned.child {
                    assert!(module.isomorphisms(cloned));
                    assert_ne!(module.id, cloned.id);
                    let module_children = &module.children;
                    let cloned_children = &cloned.children;
                    assert_eq!(module_children.len(), cloned_children.len());
                    for (inner_module, inner_cloned) in module_children.iter().zip(cloned_children) {
                        assert!(inner_module.isomorphisms(inner_cloned));
                        assert_ne!(inner_module.id, inner_cloned.id);
                        assert_eq!(inner_cloned.parent, Some(cloned.id));
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

    if let ProgramModuleChildItems::MultiBlock(module_lists) = &module.child {
        if let ProgramModuleChildItems::MultiBlock(cloned_lists) = &cloned.child {
            assert_eq!(module_lists.len(), cloned_lists.len());
            for (module_list, cloned_list) in module_lists.iter().zip(cloned_lists) {
                assert!(module_list.isomorphisms(cloned_list));
                assert_ne!(module_list.id, cloned_list.id);
                assert_eq!(cloned_list.parent, Some(cloned.id));
                let module_children = &module_list.children;
                let cloned_children = &cloned_list.children;
                assert_eq!(module_children.len(), cloned_children.len());
                for (inner_module, inner_cloned) in module_children.iter().zip(cloned_children) {
                    assert!(inner_module.isomorphisms(inner_cloned));
                    assert_ne!(inner_module.id, inner_cloned.id);
                    assert_eq!(inner_cloned.parent, Some(cloned_list.id));
                }
            }
        } else { unreachable!(); }
    } else { unreachable!(); }
}

#[test]
fn program_module_get_module_test() {
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
        ));
    assert_eq!(module.get_module(Uuid::new_v4()), Err(DotEveryEditorErrorMessage::NotFound));
    assert_eq!(module.get_module(module.id), Ok(module.clone()));
    let (list_id, module_id, module_obj) = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        if let ProgramModuleChildItems::Block(list) = &module.child {
            (list.id, list.children[3].id, list.children[3].clone())
        } else { unreachable!(); }
    } else { unreachable!(); };
    assert_eq!(module.get_module(list_id), Err(DotEveryEditorErrorMessage::ModuleToGetMustBeProgramModule));
    assert_eq!(module.get_module(module_id), Ok(module_obj));

    let (list_id, module_id, module_obj) = if let ProgramModuleChildItems::MultiBlock(lists) = &module.child {
        (lists[3].id, lists[3].children[3].id, lists[3].children[3].clone())
    } else { unreachable!(); };
    assert_eq!(module.get_module(list_id), Err(DotEveryEditorErrorMessage::ModuleToGetMustBeProgramModule));
    assert_eq!(module.get_module(module_id), Ok(module_obj));
}

#[test]
fn program_module_add_test() {
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
    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, 0, add), Err(DotEveryEditorErrorMessage::OptionDoesNotExpectProgramModule));

    let mut module = new_module();
    let add = new_module();
    assert_eq!(module.add(module.id, 2, add), Err(DotEveryEditorErrorMessage::CanNotReplace));

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let _uuid = add.id.clone();
    let mut module1 = add.clone();
    module1.parent = Some(module.id);
    expect.options[3] = ProgramModuleOption::ProgramModule(Some(module1));
    assert_eq!(module.add(module.id, 3, add), Ok(()));
    assert_eq!(module, expect);
    if let ProgramModuleOption::ProgramModule(Some(option)) = &module.options[3] {
        assert_eq!(option.parent, Some(module.id));
    } else { unreachable!(); }

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        if let ProgramModuleChildItems::Block(list) = &module.child {
            list.id
        } else { unreachable!(); }
    } else { unreachable!(); };
    if let ProgramModuleOption::ProgramModule(Some(module)) = &mut expect.options[2] {
        if let ProgramModuleChildItems::Block(list) = &mut module.child {
            let mut module1 = add.clone();
            module1.parent = Some(list.id);
            list.children.insert(3, module1);
        } else { unreachable!(); }
    } else { unreachable!(); }
    assert_eq!(module.add(id, 3, add), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let mut expect = module.clone();
    let add = new_module();
    let id = if let ProgramModuleChildItems::MultiBlock(lists) = &module.child {
        lists[3].id
    } else { unreachable!(); };
    if let ProgramModuleChildItems::MultiBlock(lists) = &mut expect.child {
        let mut module1 = add.clone();
        module1.parent = Some(lists[3].id);
        lists[3].children.insert(3, module1);
    } else { unreachable!(); }
    assert_eq!(module.add(id, 3, add), Ok(()));
    assert_eq!(module, expect);
}

#[test]
fn program_module_remove_test() {
    let new_module: fn() -> ProgramModule = || {
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
                                            vec![ProgramModuleOption::ProgramModule(
                                                Some(
                                                    ProgramModule::new(
                                                        Vec::new(),
                                                        ProgramModuleChildItems::None)))],
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
    let id = if let ProgramModuleChildItems::MultiBlock(lists) = &mut module.child {
        lists[0].id
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleChildItems::MultiBlock(lists) = &mut expect.child {
        lists.remove(0);
    } else { unreachable!(); }
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let id = if let ProgramModuleChildItems::MultiBlock(lists) = &mut module.child {
        lists[3].children[3].id
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleChildItems::MultiBlock(lists) = &mut expect.child {
        lists[3].children.remove(3);
    } else { unreachable!(); }
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);

    let mut module = new_module();
    let id = if let ProgramModuleOption::ProgramModule(Some(module)) = &module.options[2] {
        if let ProgramModuleChildItems::Block(list) = &module.child {
            list.id
        } else { unreachable!(); }
    } else { unreachable!(); };
    let mut expect = module.clone();
    if let ProgramModuleOption::ProgramModule(Some(module)) = &mut expect.options[2] {
        if let ProgramModuleChildItems::Block(list) = &mut module.child {
            list.children.clear();
        } else { unreachable!(); }
    } else { unreachable!(); }
    assert_eq!(module.remove(id), Ok(()));
    assert_eq!(module, expect);
}

//TODO:
//DragModuleAgentでホバー中の場所のインデックスを保持するようにする
//メインのAgentとの連携
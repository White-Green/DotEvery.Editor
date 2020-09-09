use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::util::Isomorphism;

#[test]
fn module_isomorphisms_success_test() {
    let a = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::None);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::StringSign("test".to_string()), ProgramModuleOption::StringInput("test1".to_string()), ProgramModuleOption::ProgramModule(None), ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::StringSign("test".to_string()), ProgramModuleOption::StringInput("test2".to_string()), ProgramModuleOption::ProgramModule(None), ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::BlockVertical(Vec::new()));
    let b = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::BlockVertical(Vec::new()));
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::BlockHorizontal(Vec::new()));
    let b = ProgramModule::<i32>::new_default(Vec::new(), ProgramModuleChildItems::BlockHorizontal(Vec::new()));
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::<i32>::new(Vec::new(), ProgramModuleChildItems::None, 10);
    let b = ProgramModule::<i32>::new(Vec::new(), ProgramModuleChildItems::None, 10);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));
}

#[test]
fn module_isomorphisms_failed_test() {
    let a = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(vec![], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::StringSign("test1".to_string())], ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::StringSign("test2".to_string())], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::ProgramModule(None)], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));


    let a = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::<i32>::new_default(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new_default(vec![ProgramModuleOption::StringSign("test".to_string())], ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModule::<i32>::new(Vec::new(), ProgramModuleChildItems::None, 1);
    let b = ProgramModule::<i32>::new(Vec::new(), ProgramModuleChildItems::None, 2);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));
}
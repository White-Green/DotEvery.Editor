use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::logic::program_module_list::ProgramModuleList;
use crate::util::Isomorphism;

#[test]
fn module_isomorphisms_success_test() {
    let a = ProgramModule::new(Vec::new(), ProgramModuleChildItems::None);
    let b = ProgramModule::new(Vec::new(), ProgramModuleChildItems::None);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::new(vec![ProgramModuleOption::StringSign("test".to_string()), ProgramModuleOption::StringInput("test1".to_string()), ProgramModuleOption::ProgramModule(None), ProgramModuleOption::ProgramModule(Some(ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::new(vec![ProgramModuleOption::StringSign("test".to_string()), ProgramModuleOption::StringInput("test2".to_string()), ProgramModuleOption::ProgramModule(None), ProgramModuleOption::ProgramModule(Some(ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::new(Vec::new(), ProgramModuleChildItems::Block(ProgramModuleList::new(Vec::new())));
    let b = ProgramModule::new(Vec::new(), ProgramModuleChildItems::Block(ProgramModuleList::new(Vec::new())));
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));

    let a = ProgramModule::new(Vec::new(), ProgramModuleChildItems::MultiBlock(vec![ProgramModuleList::new(Vec::new())]));
    let b = ProgramModule::new(Vec::new(), ProgramModuleChildItems::MultiBlock(vec![ProgramModuleList::new(Vec::new())]));
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(b.isomorphisms(&a));
    assert!(a.isomorphisms(&a));
}

#[test]
fn module_isomorphisms_failed_test() {
    let a = ProgramModule::new(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::new(vec![], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModule::new(vec![ProgramModuleOption::StringSign("test1".to_string())], ProgramModuleChildItems::None);
    let b = ProgramModule::new(vec![ProgramModuleOption::StringSign("test2".to_string())], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModule::new(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::new(vec![ProgramModuleOption::ProgramModule(None)], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));


    let a = ProgramModule::new(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    let b = ProgramModule::new(vec![ProgramModuleOption::ProgramModule(Some(ProgramModule::new(vec![ProgramModuleOption::StringSign("test".to_string())], ProgramModuleChildItems::None)))], ProgramModuleChildItems::None);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));
}

#[test]
fn list_isomorphisms_success_test() {
    let a = ProgramModuleList::new(Vec::new());
    let b = ProgramModuleList::new(Vec::new());
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(a.isomorphisms(&a));
    assert!(b.isomorphisms(&a));

    let a = ProgramModuleList::new(vec![ProgramModule::new(Vec::new(), ProgramModuleChildItems::None), ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    let b = ProgramModuleList::new(vec![ProgramModule::new(Vec::new(), ProgramModuleChildItems::None), ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    assert_ne!(a, b);
    assert!(a.isomorphisms(&b));
    assert!(a.isomorphisms(&a));
    assert!(b.isomorphisms(&a));
}

#[test]
fn list_isomorphisms_failed_test() {
    let a = ProgramModuleList::new(vec![ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    let b = ProgramModuleList::new(vec![ProgramModule::new(Vec::new(), ProgramModuleChildItems::None), ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));

    let a = ProgramModuleList::new(vec![ProgramModule::new(Vec::new(), ProgramModuleChildItems::None), ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    let b = ProgramModuleList::new(vec![ProgramModule::new(vec![ProgramModuleOption::StringSign("test".to_string())], ProgramModuleChildItems::None), ProgramModule::new(Vec::new(), ProgramModuleChildItems::None)]);
    assert!(!a.isomorphisms(&b));
    assert!(!b.isomorphisms(&a));
}
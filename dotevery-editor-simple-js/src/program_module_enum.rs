use enum_iterator::IntoEnumIterator;

use dotevery_editor_lib::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};

#[derive(Clone, PartialEq)]
pub enum ProgramModuleType {
    Print,
    StringLiteral,
    NumberLiteral,
    Variable(String),
    Switch,
    Case,
    DefaultCase,
    ValueAssign,
    ValueAdd,
    ValueSub,
    ValueMul,
    ValueDiv,
    ValueRem,
}

fn create_module_print() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("print")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::Print,
    )
}

fn create_module_string_literal() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("\"")), ProgramModuleOption::StringInput(String::new()), ProgramModuleOption::StringSign(String::from("\""))],
        ProgramModuleChildItems::None,
        ProgramModuleType::StringLiteral,
    )
}

fn create_module_number_literal() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("数値")), ProgramModuleOption::StringInput(String::new())],
        ProgramModuleChildItems::None,
        ProgramModuleType::NumberLiteral,
    )
}

fn create_module_variable(s: String) -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(s.clone())],
        ProgramModuleChildItems::None,
        ProgramModuleType::Variable(s),
    )
}

fn create_module_switch() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("switch")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::BlockHorizontal(Vec::new()),
        ProgramModuleType::Switch,
    )
}

fn create_module_case() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("case")), ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from(":"))],
        ProgramModuleChildItems::BlockVertical(Vec::new()),
        ProgramModuleType::Case,
    )
}

fn create_module_default_case() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::StringSign(String::from("default:"))],
        ProgramModuleChildItems::BlockVertical(Vec::new()),
        ProgramModuleType::DefaultCase,
    )
}

fn create_module_value_assign() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("=")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueAssign,
    )
}

fn create_module_value_add() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("＋")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueAdd,
    )
}

fn create_module_value_sub() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("－")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueSub,
    )
}

fn create_module_value_mul() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("×")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueMul,
    )
}

fn create_module_value_div() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("÷")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueDiv,
    )
}

fn create_module_value_rem() -> ProgramModule<ProgramModuleType> {
    ProgramModule::new(
        vec![ProgramModuleOption::ProgramModule(None), ProgramModuleOption::StringSign(String::from("mod")), ProgramModuleOption::ProgramModule(None)],
        ProgramModuleChildItems::None,
        ProgramModuleType::ValueRem,
    )
}


pub fn create_module(t: ProgramModuleType) -> ProgramModule<ProgramModuleType> {
    match t {
        ProgramModuleType::Print => create_module_print(),
        ProgramModuleType::StringLiteral => create_module_string_literal(),
        ProgramModuleType::NumberLiteral => create_module_number_literal(),
        ProgramModuleType::Variable(s) => create_module_variable(s),
        ProgramModuleType::Switch => create_module_switch(),
        ProgramModuleType::Case => create_module_case(),
        ProgramModuleType::DefaultCase => create_module_default_case(),
        ProgramModuleType::ValueAssign => create_module_value_assign(),
        ProgramModuleType::ValueAdd => create_module_value_add(),
        ProgramModuleType::ValueSub => create_module_value_sub(),
        ProgramModuleType::ValueMul => create_module_value_mul(),
        ProgramModuleType::ValueDiv => create_module_value_div(),
        ProgramModuleType::ValueRem => create_module_value_rem(),
    }
}
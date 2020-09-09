use std::sync::{Arc, RwLock};

use enum_iterator::IntoEnumIterator;
use yew::agent::HandlerId;

use dotevery_editor_lib::clog;
use dotevery_editor_lib::components::DotEveryBridge;
use dotevery_editor_lib::logic::dotevery_editor::DotEveryEditor;
use dotevery_editor_lib::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};
use dotevery_editor_lib::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};

use crate::program_module_enum::{create_module, ProgramModuleType};

pub struct Controller {
    bridge: DotEveryBridge<Self, ProgramModuleType>,
    data: Arc<RwLock<DotEveryEditor<ProgramModuleType>>>,
    palette: Arc<RwLock<Vec<ProgramModule<ProgramModuleType>>>>,
}

pub enum ControllerInput {
    RequestUpdateLogicData,
    AddVariable(String),
    RemoveVariable(String),
}

pub enum ControllerOutput {
    UpdateLogicData(DotEveryEditor<ProgramModuleType>),
}

impl DotEveryEditorController<ProgramModuleType> for Controller {
    type Input = ControllerInput;
    type Output = ControllerOutput;

    fn create(data: Arc<RwLock<DotEveryEditor<ProgramModuleType>>>, palette: Arc<RwLock<Vec<ProgramModule<ProgramModuleType>>>>, bridge: DotEveryBridge<Self, ProgramModuleType>) -> Self {
        clog!("Controller created");

        let types = [
            ProgramModuleType::Print,
            ProgramModuleType::NumberLiteral,
            ProgramModuleType::StringLiteral,
            ProgramModuleType::Switch,
            ProgramModuleType::Case,
            ProgramModuleType::DefaultCase,
            ProgramModuleType::ValueAssign,
            ProgramModuleType::ValueAdd,
            ProgramModuleType::ValueSub,
            ProgramModuleType::ValueMul,
            ProgramModuleType::ValueDiv,
            ProgramModuleType::ValueRem,
        ];
        let palette_data: Vec<_> = types.iter().map(|t| create_module(t.clone())).collect();
        // data.write().unwrap().list = palette_data.clone();
        // data.write().unwrap().list.push(palette_data[0].deep_clone());
        *palette.write().unwrap() = palette_data;
        Self {
            bridge,
            data,
            palette,
        }
    }

    fn update(&mut self) {
        // clog!(format!("update data"));
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            ControllerInput::RequestUpdateLogicData => {
                self.bridge.respond(id, Self::Output::UpdateLogicData(self.data.read().unwrap().clone()));
            }
            ControllerInput::AddVariable(s) => {
                let mut palette = self.palette.write().unwrap();
                palette.push(create_module(ProgramModuleType::Variable(s)));
                self.bridge.notify_update(DotEveryEditorCommand::UpdatePalette);
            }
            ControllerInput::RemoveVariable(s) => {
                let mut palette = self.palette.write().unwrap();
                let index = palette.iter().enumerate().find_map(|(i, module)| {
                    if module.type_data == ProgramModuleType::Variable(s.clone()) {
                        Some(i)
                    } else {
                        None
                    }
                });
                if let Some(index) = index {
                    palette.remove(index);
                    self.bridge.notify_update(DotEveryEditorCommand::UpdatePalette);
                }
                let mut data = self.data.write().unwrap();
                let variable_modules = data.get_modules_by_type(&ProgramModuleType::Variable(s));
                let variable_modules_id = variable_modules.into_iter().map(|module| module.id()).collect::<Vec<_>>();
                for module in &variable_modules_id {
                    data.remove(*module);
                }
                if variable_modules_id.len() > 0 {
                    self.bridge.notify_update(DotEveryEditorCommand::Update);
                }
            }
        }
    }
}

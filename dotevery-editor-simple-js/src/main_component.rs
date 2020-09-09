use std::collections::HashSet;

use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{Bridge, Component, ComponentLink, Html, html, NodeRef, Properties};

use dotevery_editor_lib::clog;
use dotevery_editor_lib::components::dotevery_editor::DotEveryEditorComponent;
use dotevery_editor_lib::components::DotEveryEditorControllerBridge;
use dotevery_editor_lib::logic::dotevery_editor::DotEveryEditor;
use dotevery_editor_lib::logic::dotevery_editor_controller::DotEveryEditorController;
use dotevery_editor_lib::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};

use crate::controller::{Controller, ControllerInput, ControllerOutput};
use crate::ProgramModuleType;

#[derive(Clone, Default, Properties)]
pub struct MainComponentProperties {}

pub struct MainComponent {
    props: MainComponentProperties,
    link: ComponentLink<Self>,
    controller_bridge: DotEveryEditorControllerBridge<Controller, ProgramModuleType>,
    variables: HashSet<String>,
    variable_input_ref: NodeRef,
    variable_list_ref: NodeRef,
    compile_result: String,
    exec_result: String,
}

pub enum MainComponentMessage {
    Run,
    AddVariable,
    RemoveVariable,
    MsgFromController(ControllerOutput),
}

impl Component for MainComponent {
    type Message = MainComponentMessage;
    type Properties = MainComponentProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let bridge = Controller::bridge(link.callback(|msg| Self::Message::MsgFromController(msg)));
        Self {
            props,
            link,
            controller_bridge: bridge,
            variables: HashSet::new(),
            variable_input_ref: NodeRef::default(),
            variable_list_ref: NodeRef::default(),
            compile_result: String::new(),
            exec_result: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            MainComponentMessage::Run => {
                clog!("run clicked");
                self.controller_bridge.send(ControllerInput::RequestUpdateLogicData);
                false
            }
            Self::Message::AddVariable => {
                if let Some(element) = self.variable_input_ref.cast::<HtmlInputElement>() {
                    let variable_name: String = element.value();
                    element.set_value("");
                    let variable_name = variable_name.replace(" ", "");
                    let variable_name = variable_name.replace("　", "");
                    if self.variables.insert(variable_name.clone()) {
                        self.controller_bridge.send(ControllerInput::AddVariable(variable_name));
                        true
                    } else {
                        false
                    }
                } else {
                    clog!("variable_input_ref is not HtmlInputElement");
                    false
                }
            }
            Self::Message::RemoveVariable => {
                if let Some(element) = self.variable_list_ref.cast::<HtmlSelectElement>() {
                    let variable_name = element.value();
                    self.variables.remove(&variable_name);
                    self.controller_bridge.send(ControllerInput::RemoveVariable(variable_name));
                    true
                } else {
                    clog!("variable_list_ref is not HtmlSelectElement");
                    false
                }
            }
            MainComponentMessage::MsgFromController(msg) => {
                match msg {
                    ControllerOutput::UpdateLogicData(logic) => {
                        clog!("run");
                        let result = match compile(logic, &self.variables) {
                            Ok((prefix, code, suffix)) => {
                                self.compile_result = code.clone();
                                Some(format!("{}{}{}", prefix, code, suffix))
                            }
                            Err(msg) => {
                                self.compile_result = match msg {
                                    CompileError::NeedProgramModule(traceback) => {
                                        format!("NeedProgramModuleError\n{}", traceback)
                                    }
                                    e => format!("{:?}", e)
                                };
                                self.exec_result = String::new();
                                None
                            }
                        };
                        clog!(format ! ("{:?}", &result));
                        if let Some(code) = result {
                            self.exec_result = exec(&code);
                        }
                        clog!(&self.exec_result);
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let variables = self.variables.iter().map(|s| {
            html! {<option>{s}</option>}
        });
        html! {
            <div class = "sample_main container">
                <button class ="btn btn-primary" onclick = self.link.callback( |_ | Self::Message::Run)>{"Compile & Run"}</button>
                <div>
                    <div class= "form-group form-inline">
                        <div class = "form-group">
                            <input ref = self.variable_input_ref.clone() type = "text" class = "form-control" placeholder = "Variable name" />
                        </div>
                        <div class = "form-group">
                            <button class = "btn btn-primary" onclick = self.link.callback( | _ | Self::Message::AddVariable)>{"追加"}</button>
                        </div>
                    </div>
                    <div class = "form-group">
                        <select ref = self.variable_list_ref.clone() multiple= true class = "form-control">
                            {for variables}
                        </select>
                    </div>
                    <button class = "btn btn-primary" onclick = self.link.callback( | _ | Self::Message::RemoveVariable)>{"削除"}</button>
                </div>
                <div class = "editor_area">
                    <DotEveryEditorComponent <Controller, ProgramModuleType> />
                    <div class = "result_area">
                        <pre class = "border m-3 p-2"> <code>{self.compile_result.clone()}</code> </pre>
                        <pre class = "border m-3 p-2"> <samp>{self.exec_result.clone()}</samp> </pre>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Debug)]
enum CompileError {
    ModuleStructureError,
    NeedProgramModule(String),
}

fn compile(data: DotEveryEditor<ProgramModuleType>, variables: &HashSet<String>) -> Result<(String, String, String), CompileError> {
    let mut prefix = String::from("(()=>{");
    prefix.push_str("\nlet console={log:function(v){console.buffer=console.buffer+v+\"\\n\";},buffer:\"\"};\n");
    let mut s = String::new();
    for variable in variables {
        s.push_str(&format!("let {} = undefined;\n", variable));
    }
    for (i, module) in data.list.iter().enumerate() {
        match compile_inner(module) {
            Ok(c) => { s.push_str(&format!("{}\n", c)); }
            Err(CompileError::NeedProgramModule(traceback)) => {
                return Err(CompileError::NeedProgramModule(format!("at block[{}]\n{}", i, traceback)));
            }
            Err(err) => { return Err(err); }
        }
    }

    let mut suffix = format!("return console.buffer;");
    suffix.push_str("\n})()");
    Ok((prefix, s, suffix))
}

fn compile_inner(module: &ProgramModule<ProgramModuleType>) -> Result<String, CompileError> {
    match &module.type_data {
        ProgramModuleType::Print => {
            match &module.options.get(1) {
                Some(ProgramModuleOption::ProgramModule(Some(inner_module))) => {
                    match compile_inner(&inner_module) {
                        Ok(s) => { Ok(format!("console.log({});", s)) }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            Err(CompileError::NeedProgramModule(format!("at print argument\n{}", traceback)))
                        }
                        e => e
                    }
                }
                Some(ProgramModuleOption::ProgramModule(None)) => {
                    Ok(format!("console.log(\"\");"))
                }
                _ => { Err(CompileError::ModuleStructureError) }
            }
        }
        ProgramModuleType::StringLiteral => {
            if let Some(ProgramModuleOption::StringInput(s)) = &module.options.get(1) {
                let s = s.replace("\"", "\\\"");
                Ok(format!("\"{}\"", s))
            } else {
                Err(CompileError::ModuleStructureError)
            }
        }
        ProgramModuleType::NumberLiteral => {
            if let Some(ProgramModuleOption::StringInput(s)) = &module.options.get(1) {
                Ok(format!("parseFloat(\"{}\")", s))
            } else {
                Err(CompileError::ModuleStructureError)
            }
        }
        ProgramModuleType::Variable(s) => {
            Ok(s.clone())
        }
        ProgramModuleType::Switch => {
            let prefix = if let Some(ProgramModuleOption::ProgramModule(module)) = module.options.get(1) {
                if let Some(module) = module {
                    match compile_inner(module) {
                        Ok(s) => { format!("switch ({}) {{", s) }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            return Err(CompileError::NeedProgramModule(format!("at switch argument\n{}", traceback)));
                        }
                        e => return e,
                    }
                } else {
                    return Err(CompileError::NeedProgramModule(String::from("at switch argument")));
                }
            } else {
                return Err(CompileError::ModuleStructureError);
            };
            let code = if let ProgramModuleChildItems::BlockHorizontal(list) = &module.child {
                let mut code = String::new();
                for (i, module) in list.iter().enumerate() {
                    match compile_inner(module) {
                        Ok(s) => {
                            code.push_str(&s);
                            code.push('\n');
                        }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            return Err(CompileError::NeedProgramModule(format!("at switch block[{}]\n{}", i, traceback)));
                        }
                        e => return e,
                    }
                }
                code
            } else {
                return Err(CompileError::ModuleStructureError);
            };
            let suffix = String::from("}");
            Ok(format!("{}\n{}{}", prefix, code, suffix))
        }
        ProgramModuleType::Case => {
            let prefix = if let Some(ProgramModuleOption::ProgramModule(module)) = module.options.get(1) {
                if let Some(module) = module {
                    match compile_inner(module) {
                        Ok(s) => { format!("case {}:", s) }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            return Err(CompileError::NeedProgramModule(format!("at case argument\n{}", traceback)));
                        }
                        e => return e,
                    }
                } else {
                    return Err(CompileError::NeedProgramModule(String::from("at case argument")));
                }
            } else {
                return Err(CompileError::ModuleStructureError);
            };
            let code = if let ProgramModuleChildItems::BlockVertical(list) = &module.child {
                let mut code = String::new();
                for (i, module) in list.iter().enumerate() {
                    match compile_inner(module) {
                        Ok(s) => {
                            code.push_str(&s);
                            code.push('\n');
                        }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            return Err(CompileError::NeedProgramModule(format!("at case block[{}]\n{}", i, traceback)));
                        }
                        e => return e,
                    }
                }
                code
            } else {
                return Err(CompileError::ModuleStructureError);
            };
            let suffix = String::from("break;");
            Ok(format!("{}\n{}{}", prefix, code, suffix))
        }
        ProgramModuleType::DefaultCase => {
            let prefix = String::from("default:");
            let code = if let ProgramModuleChildItems::BlockVertical(list) = &module.child {
                let mut code = String::new();
                for (i, module) in list.iter().enumerate() {
                    match compile_inner(module) {
                        Ok(s) => {
                            code.push_str(&s);
                            code.push('\n');
                        }
                        Err(CompileError::NeedProgramModule(traceback)) => {
                            return Err(CompileError::NeedProgramModule(format!("at default block[{}]\n{}", i, traceback)));
                        }
                        e => return e,
                    }
                }
                code
            } else {
                return Err(CompileError::ModuleStructureError);
            };
            let suffix = String::from("break;");
            Ok(format!("{}\n{}{}", prefix, code, suffix))
        }
        ProgramModuleType::ValueAssign => match compile_binary_operator(&module, "=", "assignment") {
            Ok(s) => { Ok(format!("{};", s)) }
            e => e
        },
        ProgramModuleType::ValueAdd => compile_binary_operator(&module, "+", "add"),
        ProgramModuleType::ValueSub => compile_binary_operator(&module, "-", "sub"),
        ProgramModuleType::ValueMul => compile_binary_operator(&module, "*", "multiply"),
        ProgramModuleType::ValueDiv => compile_binary_operator(&module, "/", "divide"),
        ProgramModuleType::ValueRem => compile_binary_operator(&module, "%", "mod"),
    }
}

fn compile_binary_operator(module: &&ProgramModule<ProgramModuleType>, operator: &str, name: &str) -> Result<String, CompileError> {
    let left = if let ProgramModuleOption::ProgramModule(inner_module) = &module.options[0] {
        if let Some(inner_module) = inner_module {
            match compile_inner(&inner_module) {
                Ok(s) => { s }
                Err(CompileError::NeedProgramModule(traceback)) => {
                    return Err(CompileError::NeedProgramModule(format!("at {} operator left argument\n{}", name, traceback)));
                }
                e => return e,
            }
        } else {
            return Err(CompileError::NeedProgramModule(format!("at {} operator left argument", name)));
        }
    } else {
        return Err(CompileError::ModuleStructureError);
    };
    let right = if let ProgramModuleOption::ProgramModule(inner_module) = &module.options[2] {
        if let Some(inner_module) = inner_module {
            match compile_inner(&inner_module) {
                Ok(s) => { s }
                Err(CompileError::NeedProgramModule(traceback)) => {
                    return Err(CompileError::NeedProgramModule(format!("at {} operator right argument\n{}", name, traceback)));
                }
                e => return e,
            }
        } else {
            return Err(CompileError::NeedProgramModule(format!("at {} operator right argument", name)));
        }
    } else {
        return Err(CompileError::ModuleStructureError);
    };
    Ok(format!("({}) {} ({})", left, operator, right))
}

fn exec(code: &str) -> String {
    match js_sys::eval(code) {
        Ok(val) | Err(val) => {
            let object = js_sys::Object::from(val);
            let object = object.to_string();
            let val = JsValue::from(object);
            match val.as_string() {
                Some(s) => s,
                None => String::from("cast error"),
            }
        }
    }
}

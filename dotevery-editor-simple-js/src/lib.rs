#![recursion_limit = "1024"]
extern crate wee_alloc;

use std::sync::{Arc, RwLock};

use wasm_bindgen::prelude::*;
use yew::agent::HandlerId;
use yew::prelude::*;

use dotevery_editor_lib::clog;
use dotevery_editor_lib::components::dotevery_editor::DotEveryEditorComponent;
use dotevery_editor_lib::components::DotEveryBridge;
use dotevery_editor_lib::logic::dotevery_editor::DotEveryEditor;
use dotevery_editor_lib::logic::dotevery_editor_controller::{DotEveryEditorCommand, DotEveryEditorController};
use dotevery_editor_lib::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};

use crate::controller::Controller;
use crate::main_component::MainComponent;
use crate::program_module_enum::ProgramModuleType;

// use dotevery_editor_lib::logic::program_module_list::ProgramModuleList;
pub mod controller;
pub mod main_component;
pub mod program_module_enum;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    clog!("Hello,wasm world!");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(entry) = document.get_element_by_id("app") {
        App::<MainComponent>::new().mount(entry);
    } else {
        clog!("entry point element is not found.");
    }
}


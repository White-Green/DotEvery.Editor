use std::collections::HashMap;
use std::marker::PhantomData;

use either::Either;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement, window};
use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

use crate::components::dotevery_editor_agent::{DotEveryEditorAgent, DotEveryEditorAgentInputMessage};
use crate::components::drag_module_agent::{DragModuleAgent, DragModuleAgentInputMessage, DragModuleAgentOutputMessage};
use crate::components::impl_splitter::ImplSplitter;
use crate::logic::dotevery_editor::DotEveryEditorOperationIndex;
use crate::logic::dotevery_editor_controller::DotEveryEditorController;
use crate::logic::program_module::{ProgramModule, ProgramModuleChildItems, ProgramModuleOption};
use crate::util::Rect;

#[derive(Clone, PartialEq)]
pub(crate) struct ProgramModuleDefault<T: 'static + Clone + PartialEq> {
    pub(crate) list: Vec<ProgramModule<T>>,
    pub(crate) parent: Uuid,
}

#[derive(Clone, Properties)]
pub(crate) struct ProgramModuleProperties<T: 'static + Clone + PartialEq> {
    pub(crate) program_module: Either<ProgramModule<T>, ProgramModuleDefault<T>>,
    pub(crate) rect_changed_callback: Callback<(Uuid, Rect)>,
}

#[derive(PartialEq)]
pub(crate) enum ProgramModuleComponentImplType {
    Default,
    CanNotDrag,
    ListOnly,
}

pub(crate) struct ProgramModuleComponentImplTypeDefault;

impl ImplSplitter<ProgramModuleComponentImplType> for ProgramModuleComponentImplTypeDefault {
    const VALUE: ProgramModuleComponentImplType = ProgramModuleComponentImplType::Default;
    type Next = ProgramModuleComponentImplTypeDefault;
}

pub(crate) struct ProgramModuleComponentImplTypeCanNotDrag;

impl ImplSplitter<ProgramModuleComponentImplType> for ProgramModuleComponentImplTypeCanNotDrag {
    const VALUE: ProgramModuleComponentImplType = ProgramModuleComponentImplType::CanNotDrag;
    type Next = ProgramModuleComponentImplTypeCanNotDrag;
}

pub(crate) struct ProgramModuleComponentImplTypeListOnly;

impl ImplSplitter<ProgramModuleComponentImplType> for ProgramModuleComponentImplTypeListOnly {
    const VALUE: ProgramModuleComponentImplType = ProgramModuleComponentImplType::ListOnly;
    type Next = ProgramModuleComponentImplTypeListOnly;
}

pub(crate) struct ProgramModuleComponent<Controller, Type, ImplType>
    where Controller: 'static + DotEveryEditorController<Type>,
          Type: 'static + Clone + PartialEq,
          ImplType: 'static + ImplSplitter<ProgramModuleComponentImplType> {
    link: ComponentLink<Self>,
    props: ProgramModuleProperties<Type>,
    child_rects: HashMap<Uuid, Rect>,
    self_ref: NodeRef,
    options_ref: NodeRef,
    child_ref: NodeRef,
    options_node_ref: Vec<NodeRef>,
    drag_module_agent_bridge: Box<dyn Bridge<DragModuleAgent<Controller, Type>>>,
    logic_agent_bridge: Box<dyn Bridge<DotEveryEditorAgent<Controller, Type>>>,
    hovering_module: Option<(i32, i32, f64, f64)>,
    hovering_index: Option<DotEveryEditorOperationIndex>,
    element_x: i32,
    element_y: i32,
    phantom_data: PhantomData<ImplType>,
}

pub(crate) enum ProgramModuleMessage {
    Ignore,
    Drag { mouse_x: i32, mouse_y: i32 },
    NoDrag,
    MoveHoveringModule { x: i32, y: i32, module_w: f64, module_h: f64 },
    LeaveHoveringModule,
    UpdateMousePosition { x: i32, y: i32 },
    UpdateInput { index: usize, value: String },
    UpdateSelfRect,
    UpdateChildRect { id: Uuid, rect: Rect },
    RegisterUuid,
}

impl<Controller, T, ImplType> Component for ProgramModuleComponent<Controller, T, ImplType>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq,
          ImplType: 'static + ImplSplitter<ProgramModuleComponentImplType> {
    type Message = ProgramModuleMessage;
    type Properties = ProgramModuleProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            |out: DragModuleAgentOutputMessage<T>|
                match out {
                    DragModuleAgentOutputMessage::UpdateDraggingModulePosition { x, y } => Self::Message::UpdateMousePosition { x, y },
                    DragModuleAgentOutputMessage::MoveHoveringModule { x, y, module_w, module_h } => Self::Message::MoveHoveringModule { x, y, module_w, module_h },
                    DragModuleAgentOutputMessage::LeaveHoveringModule => Self::Message::LeaveHoveringModule,
                    DragModuleAgentOutputMessage::RequestRegisterUuid => Self::Message::RegisterUuid,
                    DragModuleAgentOutputMessage::RequestUpdateRect => Self::Message::UpdateSelfRect,
                    _ => Self::Message::Ignore,
                }
        );
        let mut drag_module_agent_bridge = DragModuleAgent::bridge(callback);
        if ImplType::VALUE != ProgramModuleComponentImplType::CanNotDrag {
            match &props.program_module {
                Either::Left(module) => {
                    if let Some(parent) = module.parent {
                        drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: module.id, parent_id: parent });
                    } else {
                        drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetMyId(module.id));
                    }
                }
                Either::Right(ProgramModuleDefault { parent, .. }) => {
                    drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: Uuid::nil(), parent_id: *parent });
                }
            }
        }
        let callback = link.callback(|_| Self::Message::Ignore);
        let logic_agent_bridge = DotEveryEditorAgent::bridge(callback);
        let options_node_ref =
            if let Either::Left(module) = &props.program_module {
                (0..module.options.len()).map(|_| NodeRef::default()).collect()
            } else {
                Vec::new()
            };
        Self {
            link,
            props,
            child_rects: HashMap::new(),
            self_ref: NodeRef::default(),
            options_ref: NodeRef::default(),
            child_ref: NodeRef::default(),
            options_node_ref,
            drag_module_agent_bridge,
            logic_agent_bridge,
            hovering_module: None,
            hovering_index: None,
            element_x: 0,
            element_y: 0,
            phantom_data: PhantomData,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Ignore => false,
            Self::Message::Drag { mouse_x: x, mouse_y: y } => {
                if ImplType::VALUE != ProgramModuleComponentImplType::CanNotDrag {
                    if let Either::Left(module) = &self.props.program_module {
                        // clog!("call TryStartDrag");
                        let self_element = self.self_ref.cast::<Element>().unwrap();
                        let rect = self_element.get_bounding_client_rect();
                        let offset = get_page_offset();
                        self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::TryStartDrag {
                            offset_x: x - (rect.x() + offset.0).round() as i32,
                            offset_y: y - (rect.y() + offset.1).round() as i32,
                            module: module.clone(),
                        });
                    }
                }
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateMousePosition { x, y });
                false
            }
            Self::Message::NoDrag => {
                self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::EndDrag);
                false
            }
            Self::Message::MoveHoveringModule { x, y, module_w, module_h } => {
                if ImplType::VALUE != ProgramModuleComponentImplType::CanNotDrag {
                    self.hovering_module = Some((x, y, module_w, module_h));
                    let x = x as f64;
                    let y = y as f64;
                    self.hovering_index = None;
                    if let Some(element) = self.options_ref.cast::<Element>() {
                        let rect = element.get_bounding_client_rect();
                        let offset = get_page_offset();
                        if rect.x() + offset.0 <= x && x <= rect.x() + offset.0 + rect.width() &&
                            rect.y() + offset.1 <= y && y <= rect.y() + offset.1 + rect.height() {
                            self.hovering_index = self.get_options_hovering_index(x, y);
                        }
                    }
                    if self.hovering_index == None {
                        if let Some(element) = self.child_ref.cast::<Element>() {
                            let rect = element.get_bounding_client_rect();
                            let offset = get_page_offset();
                            if rect.x() + offset.0 <= x && x <= rect.x() + offset.0 + rect.width() &&
                                rect.y() + offset.1 <= y && y <= rect.y() + offset.1 + rect.height() {
                                self.hovering_index = self.get_child_hovering_index(x, y);
                            }
                        }
                    }
                    // if let Some(val) = self.hovering_index {
                    //     clog!("update hovering index", format!("Some({:?})", val));
                    // } else {
                    //     clog!("update hovering index", "None");
                    // }
                    self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateHoveringIndex(self.hovering_index.clone()));

                    self.link.send_message(Self::Message::UpdateSelfRect);
                    true
                } else {
                    false
                }
            }
            Self::Message::LeaveHoveringModule => {
                // clog!("leave");
                self.hovering_module = None;
                self.hovering_index = None;
                self.link.send_message(Self::Message::UpdateSelfRect);
                true
            }
            Self::Message::UpdateMousePosition { x, y } => {
                self.element_x = x;
                self.element_y = y;
                true
            }
            Self::Message::UpdateSelfRect => {
                if let Some(element) = self.self_ref.cast::<Element>() {
                    let rect = element.get_bounding_client_rect();
                    if ImplType::VALUE != ProgramModuleComponentImplType::CanNotDrag {
                        let page_offset = get_page_offset();
                        if let Either::Left(module) = &self.props.program_module {
                            self.props.rect_changed_callback.emit((module.id, Rect {
                                x: rect.x() + page_offset.0,
                                y: rect.y() + page_offset.1,
                                w: rect.width(),
                                h: rect.height(),
                            }));
                        }
                        self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::UpdateRect {
                            x: rect.x() + page_offset.0,
                            y: rect.y() + page_offset.1,
                            w: rect.width(),
                            h: rect.height(),
                        });
                    }
                }
                false
            }
            Self::Message::UpdateInput { index, value } => {
                if let Either::Left(module) = &self.props.program_module {
                    self.logic_agent_bridge.send(DotEveryEditorAgentInputMessage::UpdateInput { id: module.id, index, value });
                }
                false
            }
            Self::Message::UpdateChildRect { id, rect } => {
                self.child_rects.insert(id, rect);
                self.link.send_message(Self::Message::UpdateSelfRect);
                false
            }
            Self::Message::RegisterUuid => {
                // clog!("RegisterUuid");
                //self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetMyId(self.props.program_module.id));
                false
            }
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> bool {
        if self.props.program_module == props.program_module { return false; }
        if ImplType::VALUE != ProgramModuleComponentImplType::CanNotDrag {
            match &props.program_module {
                Either::Left(module) => {
                    self.options_node_ref = (0..module.options.len()).map(|_| NodeRef::default()).collect();
                    if let Some(id) = module.parent {
                        self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: module.id.clone(), parent_id: id.clone() });
                    } else {
                        self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetMyId(module.id.clone()));
                    }
                }
                Either::Right(ProgramModuleDefault { parent, .. }) => {
                    self.options_node_ref.clear();
                    self.drag_module_agent_bridge.send(DragModuleAgentInputMessage::SetParentId { my_id: Uuid::nil(), parent_id: *parent });
                }
            }
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let rect_changed_callback = self.link.callback(|(id, rect)| Self::Message::UpdateChildRect { id, rect });
        let module_to_component = move |program_module: &ProgramModule<T>| {
            let props = ProgramModuleProperties {
                program_module: Either::Left(program_module.clone()),
                rect_changed_callback: rect_changed_callback.clone(),
            };
            html! { <ProgramModuleComponent<Controller,T, ImplType::Next> with props/> }
        };
        let list_to_html = move |list: &Vec<ProgramModule<T>>, hovering: usize| {
            let mut vec = Vec::new();

            for i in 0..usize::min(hovering, list.len()) {
                vec.push(html! {<div class="program_module_placeholder"/>});
                vec.push(module_to_component(&list[i]));
            }

            if hovering > list.len() {
                vec.push(html! {<div class="program_module_placeholder"/>});
                return vec;
            }

            vec.push(html! {<div class="program_module_placeholder program_module_placeholder_hovered"/>});
            for i in hovering..list.len() {
                vec.push(module_to_component(&list[i]));
                vec.push(html! {<div class="program_module_placeholder"/>});
            }

            vec
        };
        match &self.props.program_module {
            Either::Left(module) => {
                // clog!("view left",format!("{:#?}", module.isomorphic_transform::<()>()));
                let child = match &module.child {
                    ProgramModuleChildItems::None => { html! {} }
                    ProgramModuleChildItems::BlockVertical(list) => {
                        if ImplType::VALUE != ProgramModuleComponentImplType::ListOnly || list.len() > 0 {
                            let hovering = if let Some(DotEveryEditorOperationIndex::Child(i)) = self.hovering_index { i } else { list.len() + 1 };
                            html! {
                                <div ref=self.child_ref.clone() class="program_module_child_vertical">
                                    {for list_to_html(list, hovering)}
                                </div>
                            }
                        } else { html! {} }
                    }
                    ProgramModuleChildItems::BlockHorizontal(list) => {
                        if ImplType::VALUE != ProgramModuleComponentImplType::ListOnly || list.len() > 0 {
                            let hovering = if let Some(DotEveryEditorOperationIndex::Child(i)) = self.hovering_index { i } else { list.len() + 1 };
                            html! {
                                <div ref=self.child_ref.clone() class="program_module_child_horizontal">
                                    {for list_to_html(list, hovering)}
                                </div>
                            }
                        } else { html! {} }
                    }
                };
                let mouse_move = self.link.callback(|e: MouseEvent| {
                    if e.buttons() == 1 {
                        Self::Message::Drag {
                            mouse_x: e.page_x(),
                            mouse_y: e.page_y(),
                        }
                    } else {
                        Self::Message::NoDrag
                    }
                });
                let options = module.options.iter().enumerate().map(
                    |(i, o)| match &o {
                        ProgramModuleOption::StringSign(s) => Self::render_string_sign(self.options_node_ref[i].clone(), s.clone()),
                        ProgramModuleOption::StringInput(s) => Self::render_string_input(
                            self.options_node_ref[i].clone(),
                            self.link.callback(Self::string_input_mousemove),
                            self.link.callback(Self::string_input_change(i)),
                            s.clone()),
                        ProgramModuleOption::ProgramModule(p) => self.render_program_module(i, p),
                    });
                let style = if module.child == ProgramModuleChildItems::None {
                    "height: 100%;".to_string()
                } else {
                    String::new()
                };
                let html: Html = html! {
                    <div ref=self.self_ref.clone() class="program_module">
                        // {module.id}
                        <div ref=self.options_ref.clone() style=style onmousemove=mouse_move class="program_module_options">
                            {for options}
                        </div>
                        {child}
                    </div>
                };
                html
            }
            Either::Right(ProgramModuleDefault { list, .. }) => {
                // clog!("view right",format!("{:#?}",list.iter().map(ProgramModule::isomorphic_transform).collect::<Vec<ProgramModule<()>>>()));
                let hovering = if let Some(DotEveryEditorOperationIndex::Child(i)) = self.hovering_index { i } else { list.len() + 1 };
                // clog!(format!("hovering {:?}",hovering));
                let child = html! {
                    <div ref=self.child_ref.clone() class="program_module_child_vertical">
                        {for list_to_html(list, hovering)}
                    </div>
                };
                html! {
                    <div ref=self.self_ref.clone() class="program_module">
                        {child}
                    </div>
                }
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        self.link.send_message(Self::Message::UpdateSelfRect);
    }
}


fn set_all_input_disabled(base: &Element, disabled: bool) {
    let nodes = base.query_selector_all("input").unwrap();
    for i in 0..nodes.length() {
        let node = nodes.get(i).unwrap();
        let input = node.unchecked_ref::<HtmlInputElement>();
        input.set_disabled(disabled);
    }
}

impl<Controller, T, ImplType> ProgramModuleComponent<Controller, T, ImplType>
    where Controller: 'static + DotEveryEditorController<T>,
          T: 'static + Clone + PartialEq,
          ImplType: 'static + ImplSplitter<ProgramModuleComponentImplType> {
    fn render_string_sign(node_ref: NodeRef, s: String) -> Html {
        html! {<span ref=node_ref class="program_module_option program_module_option_string_sign">{s}</span>}
    }

    fn render_string_input(node_ref: NodeRef, onmousemove: Callback<MouseEvent>, onchange: Callback<ChangeData>, value: String) -> Html {
        let disabled = ImplType::VALUE != ProgramModuleComponentImplType::Default;
        html! {<input ref=node_ref disabled=disabled onmousemove=onmousemove onchange=onchange class="program_module_option program_module_option_string_input" value=value/>}
    }

    fn string_input_mousemove(e: MouseEvent) -> ProgramModuleMessage {
        if e.buttons() == 1 { e.stop_propagation(); }
        ProgramModuleMessage::Ignore
    }

    fn string_input_change(i: usize) -> impl Fn(ChangeData) -> ProgramModuleMessage {
        move |e: ChangeData| {
            if let ChangeData::Value(s) = e {
                ProgramModuleMessage::UpdateInput { index: i, value: s }
            } else {
                ProgramModuleMessage::Ignore
            }
        }
    }

    fn render_program_module(&self, i: usize, p: &Option<ProgramModule<T>>) -> Html {
        match p {
            Some(p) => {
                let p = ProgramModuleProperties {
                    program_module: Either::Left(p.clone()),
                    rect_changed_callback: self.link.callback(|_| ProgramModuleMessage::Ignore),
                };
                let html: Html = html! {
                    <div ref=self.options_node_ref[i].clone() class="program_module_option program_module_option_module">
                        <ProgramModuleComponent<Controller, T, ImplType::Next> with p/>
                    </div>
                };
                html
            }
            None => {
                let placeholder = if self.is_hovering(self.options_node_ref[i].clone()) {
                    html! {
                        <div class="program_module_option_program_module_placeholder_hovered"/>
                    }
                } else {
                    html! {
                        <div class="program_module_option_program_module_placeholder"/>
                    }
                };
                let html: Html = html! {
                    <div ref=self.options_node_ref[i].clone() class="program_module_option program_module_option_module">
                        {placeholder}
                    </div>
                };
                html
            }
        }
    }

    fn is_hovering(&self, node_ref: NodeRef) -> bool {
        if let Some((x, y, _w, _h)) = self.hovering_module {
            if let Some(element) = node_ref.cast::<Element>() {
                let rect = element.get_bounding_client_rect();
                let offset = get_page_offset();
                if rect.x() + offset.0 <= (x as f64) && (x as f64) <= rect.x() + offset.0 + rect.width() &&
                    rect.y() + offset.1 <= (y as f64) && (y as f64) <= rect.y() + offset.1 + rect.height() {
                    return true;
                }
            }
        }
        false
    }

    fn get_options_hovering_index(&self, x: f64, y: f64) -> Option<DotEveryEditorOperationIndex> {
        if let Either::Left(module) = &self.props.program_module {
            for (i, (option, node_ref)) in module.options.iter().zip(&self.options_node_ref).enumerate() {
                if ProgramModuleOption::ProgramModule(None) == *option {
                    if let Some(element) = node_ref.cast::<Element>() {
                        let rect = element.get_bounding_client_rect();
                        let offset = get_page_offset();
                        if rect.x() + offset.0 <= x && x <= rect.x() + offset.0 + rect.width() &&
                            rect.y() + offset.1 <= y && y <= rect.y() + offset.1 + rect.height() {
                            return Some(DotEveryEditorOperationIndex::OptionAbsolute(i));
                        }
                    }
                }
            }
            None
        } else {
            None
        }
    }

    fn get_child_hovering_index(&self, x: f64, y: f64) -> Option<DotEveryEditorOperationIndex> {
        match &self.props.program_module {
            Either::Left(ProgramModule { child: ProgramModuleChildItems::None, .. }) => {
                None
            }
            Either::Left(ProgramModule { child: ProgramModuleChildItems::BlockVertical(list), .. }) | Either::Right(ProgramModuleDefault { list, .. }) => {
                for (i, module) in list.iter().enumerate() {
                    if let Some(rect) = self.child_rects.get(&module.id) {
                        if rect.center().1 > y {
                            return Some(DotEveryEditorOperationIndex::Child(i));
                        }
                    } else {
                        return None;
                    }
                }
                Some(DotEveryEditorOperationIndex::Child(list.len()))
            }
            Either::Left(ProgramModule { child: ProgramModuleChildItems::BlockHorizontal(list), .. }) => {
                for (i, module) in list.iter().enumerate() {
                    if let Some(rect) = self.child_rects.get(&module.id) {
                        if rect.center().0 > x {
                            return Some(DotEveryEditorOperationIndex::Child(i));
                        }
                    } else {
                        return None;
                    }
                }
                Some(DotEveryEditorOperationIndex::Child(list.len()))
            }
        }
    }
}

pub(crate) fn get_page_offset() -> (f64, f64) {
    if let Some(window) = window() {
        let x = match window.page_x_offset() {
            Ok(value) => value,
            Err(err) => {
                clog!(err);
                0.
            }
        };
        let y = match window.page_y_offset() {
            Ok(value) => value,
            Err(err) => {
                clog!(err);
                0.
            }
        };
        (x, y)
    } else {
        (0., 0.)
    }
}
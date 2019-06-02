//! This crate is used for sharing a few items between the conrod examples.
//!
//! The module contains:
//!
//! - `pub struct DemoApp` as a demonstration of some state we want to change.
//! - `pub fn gui` as a demonstration of all widgets, some of which mutate our `DemoApp`.
//! - `pub struct Ids` - a set of all `widget::Id`s used in the `gui` fn.
//!
//! By sharing these items between these examples, we can test and ensure that the different events
//! and drawing backends behave in the same manner.
#![allow(dead_code)]

#[macro_use] extern crate conrod_core;
extern crate rand;
extern crate input;

pub mod all_widgets;
pub mod canvas;
pub mod old_demo;

mod template; // Not used, but intended as a file to copy-paste new components from

use conrod_core::{
    Theme,
    Ui,
    UiCell,
    widget
};

#[derive(Clone)]
pub struct Info {
    pub name: String,
    pub size: (u32, u32),
}

impl Info {
    pub fn new(name: &str, size: (u32, u32)) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }
}

pub struct Example {
    info: Info,
    component: Box<Component>,
    theme: Box<Fn() -> Theme>,
}

impl Example {
    fn new(info: Info, component: Box<Component>) -> Self {
        Self {
            info,
            component,
            theme: Box::new(|| Theme::default()),
        }
    }

    fn with_theme(mut self, theme: Box<Fn() -> Theme>) -> Self {
        self.theme = theme;
        self
    }

    fn theme(&self) -> Theme {
        (self.theme)()
    }

    pub fn info(&self) -> &Info {
        &self.info
    }

    /// Set all `Widget`s within the User Interface.
    ///
    /// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
    /// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
    /// allocations by updating the pre-existing cached state. A new graphical `Element` is only
    /// retrieved from a `Widget` in the case that it's `State` has changed in some way.
    fn set_widgets(&mut self, ui: &mut UiCell){
        self.component.set_widgets(ui);
    }

}

pub struct Namer {
    prefix: String,
}

impl Namer {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }

    pub fn title(&self, example_name: &str) -> String {
        format!("{} - {}, [Tab] Changes Example", self.prefix, example_name)
    }
}

pub struct Manager {
    examples: Vec<Example>,
    example_id: usize,
    ui: Ui,
}

impl Manager {
    /// Initial info for window and Ui building
    /// It would be convenient if window building and UiBuilder didn't need an initial size
    pub fn info() -> Info {
        all_widgets::info()
    }
    
    pub fn new(rust_logo: conrod_core::image::Id) -> Self {
        let size = Manager::info().size;
        let ui = conrod_core::UiBuilder::new([size.0 as f64, size.1 as f64]).build();
        let mut manager = Self {
            examples: Vec::new(),
            example_id: 0,
            ui,
        };
        {
            let ui = &mut manager.ui;
            manager.examples.push(Example::new(all_widgets::info(),
                Box::new(all_widgets::Gui::new(ui, rust_logo))
            ).with_theme(Box::new(all_widgets::theme)));
            manager.examples.push(Example::new(canvas::info(),
                Box::new(canvas::Gui::new(ui))
            ));
            manager.examples.push(Example::new(old_demo::info(),
                Box::new(old_demo::Gui::new(ui))
            ));
        }
        manager
    }

    pub fn ui(&mut self) -> &mut Ui {
        &mut self.ui
    }

    pub fn example(&self) -> &Example {
        &self.examples[self.example_id]
    }

    fn update_theme(&mut self) {
        self.ui.theme = self.example().theme();
    }

    pub fn handle_event(&mut self, event: conrod_core::event::Input) -> Option<usize> {
        use conrod_core::event::Input::*;
        match event {
            Focus(_focussed) => {
            },
            Release(button) => {
                if let input::Button::Keyboard(key) = button {
                    if key == input::Key::Backquote {
                        // Change backend
                    }
                    if key == input::Key::Tab {
                        // Change example
                        self.example_id = (self.example_id + 1) % self.examples.len();
                        self.update_theme();
                        return Some(self.example_id);
                    }
                }
            },
            _ => {},
        }
        self.ui.handle_event(event);
        None
    }

    pub fn update(&mut self){
        let example = &mut self.examples[self.example_id];
        example.set_widgets(&mut self.ui.set_widgets());
    }
}

/// The environment of a Component
pub struct Env {
    canvas: widget::Id,
    last: widget::Id,
}

impl Env {
    fn new(ui: &Ui) -> Self {
        Self {
            canvas: ui.window,
            last: ui.window,
        }
    }

    fn set_canvas(&mut self, canvas: widget::Id) {
        self.canvas = canvas;
    }
    fn set_last(&mut self, last: widget::Id) {
        self.last = last;
    }

    fn get(&self) -> (widget::Id, widget::Id) {
        (self.canvas, self.last)
    }
}

/// A component that contains widgets that may mutate its state
pub trait Component {

    fn set_widgets(&mut self, ui: &mut UiCell){
        let env = Env::new(ui);
        self.update(ui, &env);
    }

    /// Set all `Widget`s within the User Interface.
    ///
    /// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
    /// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
    /// allocations by updating the pre-existing cached state. A new graphical `Element` is only
    /// retrieved from a `Widget` in the case that it's `State` has changed in some way.
    fn update(&mut self, ui: &mut UiCell, env: &Env);

    /// Returns id of widget that the next Component should be down_from
    fn get_bottom(&self) -> Option<widget::Id> { None }
}

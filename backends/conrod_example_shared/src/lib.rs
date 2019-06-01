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

mod crop_kids;
mod list;
mod nested_canvas;

#[macro_use] extern crate conrod_core;
extern crate rand;
extern crate input;

mod layout;
mod text;
mod shapes;
mod image;
mod button_xy_pad_toggle;
mod number_dialer_plotpath;

pub mod canvas;
pub mod old_demo;

mod template; // Not used, but intended as a file to copy-paste new components from

use layout::*;

use conrod_core::{
    Theme,
    Ui,
    UiCell,
    Widget,
    widget
};

#[derive(Copy, Clone)]
pub enum Example {
    New,
    Canvas,
    OldDemo,
}

impl Example {

    pub fn name(&self) -> &'static str {
        match self {
            Example::New => "All Widgets",
            Example::Canvas => "Canvas",
            Example::OldDemo => "Widget Demonstration",
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match self {
            Example::New => (600, 420),
            Example::Canvas => (800, 600),
            Example::OldDemo => (1100, 560),
        }
    }

    pub fn new_ui(&self) -> Ui {
        let (win_w, win_h) = self.size();
        conrod_core::UiBuilder::new([win_w as f64, win_h as f64]).build()
    }

    pub fn theme(&self) -> Theme {
        match self {
            Example::New => theme(),
            Example::Canvas => Theme::default(),
            Example::OldDemo => Theme::default(),
        }
    }

    pub fn next(&self) -> Example {
        match self {
            Example::New => Example::Canvas,
            Example::Canvas => Example::OldDemo,
            Example::OldDemo => Example::New,
        }
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

    pub fn title(&self, example: &Example) -> String {
        format!("{} - {}, [Tab] Changes Example", self.prefix, example.name())
    }
}

pub struct Manager {
    example: Example,
    maybe_ui: Option<Ui>,
}

impl Manager {
    pub fn new() -> Self {
        Self::new_from(Example::New)
    }

    pub fn new_from(example: Example) -> Self {
        Self {
            example,
            maybe_ui: None,
        }
    }

    pub fn example(&self) -> Example {
        self.example
    }

    pub fn ui(&mut self) -> &mut Ui {
        match self.maybe_ui {
            Some(ref mut ui) => ui,
            None => {
                let example = self.example;
                self.maybe_ui.get_or_insert_with(|| example.new_ui())
            }
        }
    }

    pub fn win_w(&self) -> u32 {
        self.example.size().0
    }

    pub fn win_h(&self) -> u32 {
        self.example.size().1
    }

    pub fn theme(&self) -> Theme {
        self.example.theme()
    }

    pub fn handle_event(&mut self, event: conrod_core::event::Input) -> Option<Example> {
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
                        self.example = self.example.next();
                        self.update_theme();
                        return Some(self.example);
                    }
                }
            },
            _ => {},
        }
        self.ui().handle_event(event);
        None
    }

    fn update_theme(&mut self) {
        self.ui().theme = self.example.theme();
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

/// A set of reasonable stylistic defaults that works for the `gui` below.
fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // Scrollbar
        canvas_scrollbar,
        // A non-scrolling overlay canvas
        overlay,
    }
}

pub struct Gui {
    ids: Ids,
    components: Vec<Box<Component>>,
    crop_kids: crop_kids::Gui,
    canvas: canvas::Gui,
    old_demo: old_demo::Gui,
}

impl Gui {
    pub fn new(manager: &mut Manager, rust_logo: conrod_core::image::Id) -> Self {
        manager.update_theme();
        let ui = manager.ui();
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            components: vec![
                Box::new(text::Gui::new(ui)),
                Box::new(shapes::Gui::new(ui)),
                Box::new(image::Gui::new(ui, rust_logo)),
                Box::new(button_xy_pad_toggle::Gui::new(ui)),
                Box::new(number_dialer_plotpath::Gui::new(ui)),
                Box::new(list::Gui::new(ui)),
                Box::new(nested_canvas::Gui::new(ui)),
            ],
            crop_kids: crop_kids::Gui::new(ui),
            canvas: canvas::Gui::new(ui),
            old_demo: old_demo::Gui::new(ui),
        }
    }

    /// Instantiate a GUI demonstrating every widget available in conrod.
    pub fn update_ui(&mut self, manager: &mut Manager) {
        //manager.update_theme();
        let ui = &mut manager.ui().set_widgets();
        let env = Env::new(ui);
        match manager.example {
            Example::New => {
                self.update_new(ui);

                // Transparent overlay canvas, the size of the window
                /*
                let window = ui.window;
                widget::Canvas::new()
                    .top_left_of(window)
                    .wh_of(window)
                    .set(self.ids.overlay, ui);*/
            },
            Example::Canvas => {
                self.canvas.update(ui, &env);
            },
            Example::OldDemo => {
                self.old_demo.update(ui, &env);
            },
        }
    }

    fn update_new(&mut self, ui: &mut UiCell){
        let ids = &self.ids;
        let canvas = self.ids.canvas;
        let mut env = Env::new(ui);
        env.set_canvas(canvas);

        // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
        // By default, its size is the size of the window. We'll use this as a background for the
        // following widgets, as well as a scrollable container for the children widgets.
        widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(canvas, ui);

        for component in self.components.iter_mut() {
            component.update(ui, &env);
            if let Some(last) = component.get_bottom() {
                env.set_last(last);
            }
        }

        // Close the scrollable region
        widget::Scrollbar::y_axis(canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);

        // Add after the scrollbar as it is draggable and will interfere with the scroll if inside it
        env.set_last(canvas);
        self.crop_kids.update(ui, &env);
    }

}

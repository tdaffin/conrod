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
    Rect,
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

/// A demonstration of some application state we want to control with a conrod GUI.
pub struct DemoApp {
    sine_frequency: f32,
    rust_logo: conrod_core::image::Id,
    old_demo: old_demo::DemoApp,
}


impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new(rust_logo: conrod_core::image::Id) -> Self {
        DemoApp {
            sine_frequency: 1.0,
            rust_logo: rust_logo,
            old_demo: old_demo::DemoApp::new(),
        }
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
    text: text::Gui,
    shapes: shapes::Gui,
    image: image::Gui,
    button_xy_pad_toggle: button_xy_pad_toggle::Gui,
    number_dialer_plotpath: number_dialer_plotpath::Gui,
    canvas: canvas::Gui,
    old_demo: old_demo::Gui,
    crop_kids: crop_kids::Gui,
    list: list::Gui,
    nested_canvas: nested_canvas::Gui,
    state: DemoApp,
}

impl Gui {
    pub fn new(manager: &mut Manager, state: DemoApp) -> Self {
        manager.update_theme();
        let ui = manager.ui();
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            text: text::Gui::new(ui),
            shapes: shapes::Gui::new(ui),
            image: image::Gui::new(ui),
            button_xy_pad_toggle: button_xy_pad_toggle::Gui::new(ui),
            number_dialer_plotpath: number_dialer_plotpath::Gui::new(ui),
            canvas: canvas::Gui::new(ui),
            old_demo: old_demo::Gui::new(ui),
            crop_kids: crop_kids::Gui::new(ui),
            list: list::Gui::new(ui),
            nested_canvas: nested_canvas::Gui::new(ui),
            state,
        }
    }

    /// Instantiate a GUI demonstrating every widget available in conrod.
    pub fn update_ui(&mut self, manager: &mut Manager) {
        //manager.update_theme();
        let ui = &mut manager.ui().set_widgets();
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
                self.canvas.update(ui);
            },
            Example::OldDemo => {
                self.old_demo.update(ui, &mut self.state.old_demo);
            },
        }
    }

    fn update_new(&mut self, ui: &mut UiCell){
        let app = &mut self.state;
        let ids = &self.ids;
        let canvas = self.ids.canvas;

        // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
        // By default, its size is the size of the window. We'll use this as a background for the
        // following widgets, as well as a scrollable container for the children widgets.
        widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(canvas, ui);

        let mut last = self.text.update(ui, canvas);

        last = self.shapes.update(ui, canvas, last);

        last = self.image.update(ui, app.rust_logo, canvas, last);

        let ball_x_range = ui.kid_area_of(canvas).unwrap().w();
        let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
        let rect = Rect::from_xy_dim([0.0, 0.0], [ball_x_range * 2.0 / 3.0, ball_y_range * 2.0 / 3.0]);
        let side = 130.0;
        
        last = self.button_xy_pad_toggle.update(ui, canvas, last, &rect, side);
        
        let space = rect.y.end - rect.y.start + side * 0.5 + MARGIN;
        self.number_dialer_plotpath.update(ui, &mut app.sine_frequency, canvas, last, space);

        self.list.update(ui);

        self.nested_canvas.update(ui);

        /////////////////////
        ///// Scrollbar /////
        /////////////////////

        widget::Scrollbar::y_axis(canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);

        // Add after the scrollbar as it is draggable and will interfere with the scroll if inside it
        self.crop_kids.update(ui);
    }

}

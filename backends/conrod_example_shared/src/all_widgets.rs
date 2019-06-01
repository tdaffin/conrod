mod layout;

mod text;
mod shapes;
mod image;
mod button_xy_pad_toggle;
mod number_dialer_plotpath;

mod crop_kids;
mod list;
mod nested_canvas;

use conrod_core::{
    Color,
    Ui,
    UiCell,
    Widget,
    color,
    widget,
};
use super::{Component, Env};
use self::layout::*;

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

pub struct GuiState {
    color: Color,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            color: color::CHARCOAL.alpha(0.25),
        }
    }
}

pub struct Gui {
    ids: Ids,
    state: GuiState,
    components: Vec<Box<Component>>,
    crop_kids: crop_kids::Gui,
}

impl Gui {
    pub fn new(ui: &mut Ui, rust_logo: conrod_core::image::Id) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            state: GuiState::new(),
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
        }
    }
}

impl Component for Gui {
    fn update(&mut self, ui: &mut UiCell, _env: &Env) {
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

    fn get_bottom(&self) -> Option<widget::Id> {
        // Return id of widget that the next Gui should be down_from
        Some(self.ids.canvas)
    }
}

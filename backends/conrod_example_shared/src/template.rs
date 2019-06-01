
use conrod_core::{
    Color,
    Colorable,
    Positionable,
    Scalar,
    Sizeable,
    Ui,
    UiCell,
    Widget,
    color,
    widget,
};
use super::{Component, Env};

const MARGIN: Scalar = 30.0;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
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
}

impl Gui {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            state: GuiState::new(),
        }
    }
}

impl Component for Gui {
    fn update(&mut self, ui: &mut UiCell, env: &Env) {
        let ids = &self.ids;
        let (canvas, last) = env.get();

        widget::Canvas::new()
            .down_from(last, MARGIN)
            .align_middle_x_of(canvas)
            .kid_area_w_of(canvas)
            .h(100.0)
            .color(self.state.color)
            .pad(MARGIN)
            .set(ids.canvas, ui);
    }

    fn get_bottom(&self) -> Option<widget::Id> {
        // Return id of widget that the next Gui should be down_from
        Some(self.ids.canvas)
    }
}

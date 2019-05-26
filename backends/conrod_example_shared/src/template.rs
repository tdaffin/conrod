
use conrod_core::{
    Color,
    Colorable,
    Positionable,
    Sizeable,
    Ui,
    UiCell,
    Widget,
    color,
    widget,
};

use layout::*;

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
}

impl Gui {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    /// Returns id of widget that the next Gui should be down_from
    pub fn update(&self, ui: &mut UiCell, state: &mut GuiState, canvas: widget::Id, last: widget::Id) -> widget::Id {
        let ids = &self.ids;

        widget::Canvas::new()
            .down_from(last, MARGIN)
            .align_middle_x_of(canvas)
            .kid_area_w_of(canvas)
            .h(100.0)
            .color(state.color)
            .pad(MARGIN)
            .set(ids.canvas, ui);

        ids.canvas // Return id of widget that the next Gui should be down_from
    }
}

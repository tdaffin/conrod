use conrod_core::{
    Positionable,
    Sizeable,
    Ui,
    UiCell,
    Widget,
    widget,
};
use super::{Component, Env};
use layout::*;

widget_ids! {
    pub struct Ids {
        image_title,
        rust_logo,
    }
}

pub struct GuiState {
    rust_logo: conrod_core::image::Id,
}

pub struct Gui {
    ids: Ids,
    state: GuiState,
}

impl Gui {
    pub fn new(ui: &mut Ui, rust_logo: conrod_core::image::Id) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            state: GuiState { rust_logo },
        }
    }
}

impl Component for Gui {
    fn update(&mut self, ui: &mut UiCell, env: &Env) {
        let ids = &self.ids;
        let (canvas, last) = env.get();

        widget::Text::new("Image")
            .down_from(last, MARGIN)
            .align_middle_x_of(canvas)
            .font_size(SUBTITLE_SIZE)
            .set(ids.image_title, ui);

        const LOGO_SIDE: conrod_core::Scalar = 144.0;
        widget::Image::new(self.state.rust_logo)
            .w_h(LOGO_SIDE, LOGO_SIDE)
            .down(60.0)
            .align_middle_x_of(canvas)
            .set(ids.rust_logo, ui);
    }

    fn get_bottom(&self) -> Option<widget::Id> {
        // Return id of widget that the next Gui should be down_from
        Some(self.ids.rust_logo)
    }
}

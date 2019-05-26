
use conrod_core::{
    Borderable,
    Colorable,
    Widget,
    Labelable,
    Positionable,
    Sizeable,
    Ui,
    UiCell,
    widget,
};

widget_ids! {
    pub struct Ids {
        section_canvas,
        section_button,
        section_text,
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

    pub fn update(&self, ui: &mut UiCell) {
        let ids = &self.ids;

        // Nest a canvas in the scroll region to illustrate a bug
        widget::Canvas::new()
            .down(10.0)
            .w_h(250.0, 200.0)
            .border(2.0) // border makes bug easier to see
            .color(conrod_core::color::LIGHT_BLUE.alpha(0.5))
            .scroll_kids() // Got to have this set for the bug to happen
            //.crop_kids() // Got to have this set for the bug to happen
            //.label("Canvas with crop_kids")
            //.label_color(conrod_core::color::DARK_ORANGE)
            .set(ids.section_canvas, ui);

        widget::Button::new()
            .parent(ids.section_canvas)
            .w_h(140.0, 40.0)
            .middle_of(ids.section_canvas)
            .label("Button")
            .set(ids.section_button, ui);

        widget::TextBox::new("Text area text")
            .w_h(140.0, 40.0)
            .down(150.0)
            .set(ids.section_text, ui);
    }
}

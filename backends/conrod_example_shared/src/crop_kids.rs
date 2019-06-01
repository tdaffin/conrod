
use conrod_core::{
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
        canvas,
        button,
        text,
    }
}

pub struct GuiState {
    pub running: bool,
    pub message: String,
}

pub struct Gui {
    ids: Ids,
    state: GuiState,
}

impl Gui {
    pub fn new(ui: &mut Ui) -> Self {
        Self {
            ids: Ids::new(ui.widget_id_generator()),
            state: GuiState {
                running: true,
                message: String::from("Edit this Text Area..."),
            },
        }
    }

    pub fn update(&mut self, ui: &mut UiCell) {
        let state = &mut self.state;
        if !state.running {
            return;
        }
        let ids = &self.ids;

        widget::Canvas::new()
            .crop_kids()
            .rgb(1.0, 1.0, 1.0)
            .w_h(300.0, 300.0)
            .label("Label")
            .label_rgb(0.0, 0.0, 0.0)
            .set(ids.canvas, ui);

        for _ in widget::Button::new()
            .parent(ids.canvas)
            .w_h(140.0, 40.0)
            .middle_of(ids.canvas)
            .label("Close")
            .set(ids.button, ui)
        {
            state.running = false;
        }

        for event in widget::TextBox::new(&state.message)
            .w_h(140.0, 40.0)
            .down(150.0)
            .set(ids.text, ui)
        {
            if let widget::text_box::Event::Update(string) = event {
                state.message = string;
            }
        }

    }
}

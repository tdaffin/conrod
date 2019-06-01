
use conrod_core::{
    Widget,
    Labelable,
    Positionable,
    Sizeable,
    Ui,
    UiCell,
    widget,
};
use super::{Component, Env};
use layout::*;

widget_ids! {
    pub struct Ids {
        dialer_title,
        number_dialer,
        plot_path,
    }
}

pub struct GuiState {
    sine_frequency: f32,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            sine_frequency: 1.0,
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
        let state = &mut self.state;
        let ids = &self.ids;

        let (canvas, last) = env.get();
        let (rect, side) = get_rect_side(ui, canvas);
        let space = rect.y.end - rect.y.start + side * 0.5 + MARGIN;

        widget::Text::new("NumberDialer and PlotPath")
            .down_from(last, space)
            .align_middle_x_of(canvas)
            .font_size(SUBTITLE_SIZE)
            .set(ids.dialer_title, ui);

        // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
        let min = 0.5;
        let max = 200.0;
        let decimal_precision = 1;
        for new_freq in widget::NumberDialer::new(state.sine_frequency, min, max, decimal_precision)
            .down(60.0)
            .align_middle_x_of(canvas)
            .w_h(160.0, 40.0)
            .label("F R E Q")
            .set(ids.number_dialer, ui)
        {
            state.sine_frequency = new_freq;
        }

        // Use the `PlotPath` widget to display a sine wave.
        let min_x = 0.0;
        let max_x = std::f32::consts::PI * 2.0 * state.sine_frequency;
        let min_y = -1.0;
        let max_y = 1.0;
        widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
            .kid_area_w_of(canvas)
            .h(240.0)
            .down(60.0)
            .align_middle_x_of(canvas)
            .set(ids.plot_path, ui);
    }

    fn get_bottom(&self) -> Option<widget::Id> {
        // Return id of widget that the next Gui should be down_from
        Some(self.ids.plot_path)
    }

}

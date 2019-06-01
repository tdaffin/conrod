use conrod_core::{ Rect, Scalar, Ui, widget };

pub const MARGIN: Scalar = 30.0;
pub const SHAPE_GAP: Scalar = 50.0;
pub const TITLE_SIZE: conrod_core::FontSize = 42;
pub const SUBTITLE_SIZE: conrod_core::FontSize = 32;

pub fn get_rect_side(ui: &Ui, canvas: widget::Id) -> (Rect, Scalar) {
    let ball_x_range = ui.kid_area_of(canvas).unwrap().w();
    let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    let rect = Rect::from_xy_dim([0.0, 0.0], [ball_x_range * 2.0 / 3.0, ball_y_range * 2.0 / 3.0]);
    let side = 130.0;
    (rect, side)
}

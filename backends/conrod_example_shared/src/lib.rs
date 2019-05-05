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

#[macro_use] extern crate conrod_core;
extern crate rand;

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;

/// A demonstration of some application state we want to control with a conrod GUI.
pub struct DemoApp {
    ball_xy: conrod_core::Point,
    ball_color: conrod_core::Color,
    sine_frequency: f32,
    rust_logo: conrod_core::image::Id,
    list: Vec<bool>,
    /// Background color (for demonstration of button and sliders).
    bg_color: conrod_core::Color,
    /// Should the button be shown (for demonstration of button).
    show_button: bool,
    /// The label that will be drawn to the Toggle.
    toggle_label: String,
    /// The number of pixels between the left side of the window
    /// and the title.
    title_pad: f64,
    /// The height of the vertical sliders (we will play with this
    /// using a number_dialer).
    v_slider_height: f64,
    /// The widget border width (we'll use this to demo Bordering
    /// and number_dialer).
    border_width: f64,
    /// Bool matrix for widget_matrix demonstration.
    bool_matrix: [[bool; 8]; 8],
    /// A vector of strings for drop_down_list demonstration.
    ddl_colors: Vec<String>,
    /// The currently selected DropDownList color.
    ddl_color: conrod_core::Color,
    /// We also need an Option<idx> to indicate whether or not an
    /// item is selected.
    selected_idx: Option<usize>,
    /// Co-ordinates for a little circle used to demonstrate the
    /// xy_pad.
    circle_pos: conrod_core::Point,
    /// Envelope for demonstration of EnvelopeEditor.
    envelopes: Vec<(Vec<conrod_core::Point>, String)>,
}


impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new(rust_logo: conrod_core::image::Id) -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod_core::color::WHITE,
            sine_frequency: 1.0,
            rust_logo: rust_logo,
            list: vec![true; 16],
            bg_color: conrod_core::color::rgb(0.2, 0.35, 0.45),
            show_button: false,
            toggle_label: "OFF".to_string(),
            title_pad: 350.0,
            v_slider_height: 230.0,
            border_width: 1.0,
            bool_matrix: [ [true, true, true, true, true, true, true, true],
                           [true, false, false, false, false, false, false, true],
                           [true, false, true, false, true, true, true, true],
                           [true, false, true, false, true, true, true, true],
                           [true, false, false, false, true, true, true, true],
                           [true, true, true, true, true, true, true, true],
                           [true, true, false, true, false, false, false, true],
                           [true, true, true, true, true, true, true, true] ],
            ddl_colors: vec!["Black".to_string(),
                              "White".to_string(),
                              "Red".to_string(),
                              "Green".to_string(),
                              "Blue".to_string()],
            ddl_color: conrod_core::color::PURPLE,
            selected_idx: None,
            circle_pos: [-50.0, 110.0],
            envelopes: vec![(vec![ [0.0, 0.0],
                                   [0.1, 17000.0],
                                   [0.25, 8000.0],
                                   [0.5, 2000.0],
                                   [1.0, 0.0], ], "Envelope A".to_string()),
                            (vec![ [0.0, 0.85],
                                   [0.3, 0.2],
                                   [0.6, 0.6],
                                   [1.0, 0.0], ], "Envelope B".to_string())],
        }
    }
}

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
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
        // The title and introduction widgets.
        title,
        introduction,
        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
        // Image.
        image_title,
        rust_logo,
        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,
        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,
        // List
        list,
        // Scrollbar
        canvas_scrollbar,
        // Old Demo
        old_canvas,
        canvas_x_scrollbar,
        canvas_y_scrollbar,
        old_title,
        old_button,
        title_pad_slider,
        old_toggle,
        red_slider,
        green_slider,
        blue_slider,
        slider_height,
        border_width,
        toggle_matrix,
        color_select,
        circle_position,
        old_circle,
        text_box_a,
        text_box_b,
        envelope_editor_a,
        envelope_editor_b,
    }
}


/// Instantiate a GUI demonstrating every widget available in conrod.
pub fn gui(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut DemoApp) {
    use conrod_core::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod_core::Scalar = 30.0;
    const SHAPE_GAP: conrod_core::Scalar = 50.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;
    const SUBTITLE_SIZE: conrod_core::FontSize = 32;

    // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    // By default, its size is the size of the window. We'll use this as a background for the
    // following widgets, as well as a scrollable container for the children widgets.
    const TITLE: &'static str = "All Widgets";
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);


    ////////////////
    ///// TEXT /////
    ////////////////


    // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    // introduction to the example.
    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);

    const INTRODUCTION: &'static str =
        "This example aims to demonstrate all widgets that are provided by conrod.\
        \n\nThe widget that you are currently looking at is the Text widget. The Text widget \
        is one of several special \"primitive\" widget types which are used to construct \
        all other widget types. These types are \"special\" in the sense that conrod knows \
        how to render them via `conrod_core::render::Primitive`s.\
        \n\nScroll down to see more widgets!";
    widget::Text::new(INTRODUCTION)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.introduction, ui);


    ////////////////////////////
    ///// Lines and Shapes /////
    ////////////////////////////


    widget::Text::new("Lines and Shapes")
        .down(70.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.shapes_title, ui);

    // Lay out the shapes in two horizontal columns.
    //
    // TODO: Have conrod provide an auto-flowing, fluid-list widget that is more adaptive for these
    // sorts of situations.
    widget::Canvas::new()
        .down(0.0)
        .align_middle_x_of(ids.canvas)
        .kid_area_w_of(ids.canvas)
        .h(360.0)
        .color(conrod_core::color::TRANSPARENT)
        .pad(MARGIN)
        .flow_down(&[
            (ids.shapes_left_col, widget::Canvas::new()),
            (ids.shapes_right_col, widget::Canvas::new()),
        ])
        .set(ids.shapes_canvas, ui);

    let shapes_canvas_rect = ui.rect_of(ids.shapes_canvas).unwrap();
    let w = shapes_canvas_rect.w();
    let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    let radius = 10.0;
    widget::RoundedRectangle::fill([w, h], radius)
        .color(conrod_core::color::CHARCOAL.alpha(0.25))
        .middle_of(ids.shapes_canvas)
        .set(ids.rounded_rectangle, ui);

    let start = [-40.0, -40.0];
    let end = [40.0, 40.0];
    widget::Line::centred(start, end).mid_left_of(ids.shapes_left_col).set(ids.line, ui);

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    widget::PointPath::centred(points).right(SHAPE_GAP).set(ids.point_path, ui);

    widget::Rectangle::fill([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_fill, ui);

    widget::Rectangle::outline([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_outline, ui);

    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    widget::Polygon::centred_fill(points).mid_left_of(ids.shapes_right_col).set(ids.trapezoid, ui);

    widget::Oval::fill([40.0, 80.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_fill, ui);

    widget::Oval::outline([80.0, 40.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_outline, ui);

    widget::Circle::fill(40.0).right(SHAPE_GAP).align_middle_y().set(ids.circle, ui);


    /////////////////
    ///// Image /////
    /////////////////


    widget::Text::new("Image")
        .down_from(ids.shapes_canvas, MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.image_title, ui);

    const LOGO_SIDE: conrod_core::Scalar = 144.0;
    widget::Image::new(app.rust_logo)
        .w_h(LOGO_SIDE, LOGO_SIDE)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.rust_logo, ui);


    /////////////////////////////////
    ///// Button, XYPad, Toggle /////
    /////////////////////////////////


    widget::Text::new("Button, XYPad and Toggle")
        .down_from(ids.rust_logo, 60.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.button_title, ui);

    let ball_x_range = ui.kid_area_of(ids.canvas).unwrap().w();
    let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    let min_x = -ball_x_range / 3.0;
    let max_x = ball_x_range / 3.0;
    let min_y = -ball_y_range / 3.0;
    let max_y = ball_y_range / 3.0;
    let side = 130.0;

    for _press in widget::Button::new()
        .label("PRESS ME")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.button_title, 60.0)
        .w_h(side, side)
        .set(ids.button, ui)
    {
        let x = rand::random::<conrod_core::Scalar>() * (max_x - min_x) - max_x;
        let y = rand::random::<conrod_core::Scalar>() * (max_y - min_y) - max_y;
        app.ball_xy = [x, y];
    }

    for (x, y) in widget::XYPad::new(app.ball_xy[0], min_x, max_x,
                                     app.ball_xy[1], min_y, max_y)
        .label("BALL XY")
        .wh_of(ids.button)
        .align_middle_y_of(ids.button)
        .align_middle_x_of(ids.canvas)
        .parent(ids.canvas)
        .set(ids.xy_pad, ui)
    {
        app.ball_xy = [x, y];
    }

    let is_white = app.ball_color == conrod_core::color::WHITE;
    let label = if is_white { "WHITE" } else { "BLACK" };
    for is_white in widget::Toggle::new(is_white)
        .label(label)
        .label_color(if is_white { conrod_core::color::WHITE } else { conrod_core::color::LIGHT_CHARCOAL })
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .align_middle_y_of(ids.button)
        .set(ids.toggle, ui)
    {
        app.ball_color = if is_white { conrod_core::color::WHITE } else { conrod_core::color::BLACK };
    }

    let ball_x = app.ball_xy[0];
    let ball_y = app.ball_xy[1] - max_y - side * 0.5 - MARGIN;
    widget::Circle::fill(20.0)
        .color(app.ball_color)
        .x_y_relative_to(ids.xy_pad, ball_x, ball_y)
        .set(ids.ball, ui);


    //////////////////////////////////
    ///// NumberDialer, PlotPath /////
    //////////////////////////////////


    widget::Text::new("NumberDialer and PlotPath")
        .down_from(ids.xy_pad, max_y - min_y + side * 0.5 + MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.dialer_title, ui);

    // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
    let min = 0.5;
    let max = 200.0;
    let decimal_precision = 1;
    for new_freq in widget::NumberDialer::new(app.sine_frequency, min, max, decimal_precision)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .w_h(160.0, 40.0)
        .label("F R E Q")
        .set(ids.number_dialer, ui)
    {
        app.sine_frequency = new_freq;
    }

    // Use the `PlotPath` widget to display a sine wave.
    let min_x = 0.0;
    let max_x = std::f32::consts::PI * 2.0 * app.sine_frequency;
    let min_y = -1.0;
    let max_y = 1.0;
    widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
        .kid_area_w_of(ids.canvas)
        .h(240.0)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.plot_path, ui);


    ////////////////
    ///// List /////
    ////////////////


    let (mut items, scrollbar) = widget::List::flow_down(app.list.len())
        .item_size(50.0)
        .scrollbar_on_top()
        .down_from(ids.plot_path, 10.0)
        .w(120.0)
        .h_of(ids.canvas)
        .set(ids.list, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let label = format!("item {}: {}", i, app.list[i]);
        let toggle = widget::Toggle::new(app.list[i])
            .label(&label)
            .label_color(conrod_core::color::WHITE)
            .color(conrod_core::color::LIGHT_BLUE);
        for v in item.set(toggle, ui) {
            app.list[i] = v;
        }
    }

    if let Some(s) = scrollbar { s.set(ui) }


    ////////////////////
    ///// Old Demo /////
    ////////////////////
    

    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        .down_from(ids.list, 10.0)
        .w_h(1100.0, 560.0)
        .border(app.border_width)
        .pad(30.0)
        .color(app.bg_color)
        .scroll_kids()
        .set(ids.old_canvas, ui);
    widget::Scrollbar::x_axis(ids.old_canvas).auto_hide(true).set(ids.canvas_y_scrollbar, ui);
    widget::Scrollbar::y_axis(ids.old_canvas).auto_hide(true).set(ids.canvas_x_scrollbar, ui);

    // Text example.
    widget::Text::new("Widget Demonstration")
        .top_left_with_margins_on(ids.old_canvas, 0.0, app.title_pad)
        .font_size(32)
        .color(app.bg_color.plain_contrast())
        .set(ids.old_title, ui);

    if app.show_button {

        // Button widget example button.
        if widget::Button::new()
            .w_h(200.0, 50.0)
            .mid_left_of(ids.old_canvas)
            .down_from(ids.old_title, 45.0)
            .rgb(0.4, 0.75, 0.6)
            .border(app.border_width)
            .label("PRESS")
            .set(ids.old_button, ui)
            .was_clicked()
        {
            app.bg_color = color::rgb(rand::random(), rand::random(), rand::random())
        }

    }

    // Horizontal slider example.
    else {

        // Create the label for the slider.
        let label = format!("Padding: {}", app.title_pad as i16);

        // Slider widget example slider(value, min, max).
        if let Some(new_pad) = widget::Slider::new(app.title_pad, 0.0, 670.0)
            .w_h(200.0, 50.0)
            .mid_left_of(ids.old_canvas)
            .down_from(ids.old_title, 45.0)
            .rgb(0.5, 0.3, 0.6)
            .border(app.border_width)
            .label(&label)
            .label_color(color::WHITE)
            .set(ids.title_pad_slider, ui)
        {
            app.title_pad = new_pad;
        }

    }

    // Keep track of the currently shown widget.
    let shown_widget = if app.show_button { ids.old_button } else { ids.title_pad_slider };

    // Toggle widget example.
    if let Some(value) = widget::Toggle::new(app.show_button)
        .w_h(75.0, 75.0)
        .down(20.0)
        .rgb(0.6, 0.25, 0.75)
        .border(app.border_width)
        .label(&app.toggle_label)
        .label_color(color::WHITE)
        .set(ids.old_toggle, ui)
        .last()
    {
        app.show_button = value;
        app.toggle_label = match value {
            true => "ON".to_string(),
            false => "OFF".to_string()
        }
    }

    macro_rules! color_slider {
        ($slider_id:ident, $bg_color:ident, $color:expr, $set_color:ident, $position:ident) => {{
            let value = app.bg_color.$bg_color();
            let label = format!("{:.*}", 2, value);
            for color in widget::Slider::new(value, 0.0, 1.0)
                .$position(25.0)
                .w_h(40.0, app.v_slider_height)
                .color($color)
                .border(app.border_width)
                .label(&label)
                .label_color(color::WHITE)
                .set(ids.$slider_id, ui)
            {
                app.bg_color.$set_color(color);
            }
        }};
    }

    color_slider!(red_slider, red, color::rgb(0.75, 0.3, 0.3), set_red, down);
    color_slider!(green_slider, green, color::rgb(0.3, 0.75, 0.3), set_green, right);
    color_slider!(blue_slider, blue, color::rgb(0.3, 0.3, 0.75), set_blue, right);

    // Number Dialer widget example. (value, min, max, precision)
    for new_height in widget::NumberDialer::new(app.v_slider_height, 25.0, 250.0, 1)
        .w_h(260.0, 60.0)
        .right_from(shown_widget, 30.0)
        .color(app.bg_color.invert())
        .border(app.border_width)
        .label("Height (px)")
        .label_color(app.bg_color.invert().plain_contrast())
        .set(ids.slider_height, ui)
    {
        app.v_slider_height = new_height;
    }

    // Number Dialer widget example. (value, min, max, precision)
    for new_width in widget::NumberDialer::new(app.border_width, 0.0, 15.0, 2)
        .w_h(260.0, 60.0)
        .down(20.0)
        .color(app.bg_color.plain_contrast().invert())
        .border(app.border_width)
        .border_color(app.bg_color.plain_contrast())
        .label("Border Width (px)")
        .label_color(app.bg_color.plain_contrast())
        .set(ids.border_width, ui)
    {
        app.border_width = new_width;
    }

    // A demonstration using widget_matrix to easily draw a matrix of any kind of widget.
    let (cols, rows) = (8, 8);
    let mut elements = widget::Matrix::new(cols, rows)
        .down(20.0)
        .w_h(260.0, 260.0)
        .set(ids.toggle_matrix, ui);

    // The `Matrix` widget returns an `Elements`, which can be used similar to an `Iterator`.
    while let Some(elem) = elements.next(ui) {
        let (col, row) = (elem.col, elem.row);

        // Color effect for fun.
        let (r, g, b, a) = (
            0.5 + (elem.col as f32 / cols as f32) / 2.0,
            0.75,
            1.0 - (elem.row as f32 / rows as f32) / 2.0,
            1.0
        );

        // We can use `Element`s to instantiate any kind of widget we like.
        // The `Element` does all of the positioning and sizing work for us.
        // Here, we use the `Element` to `set` a `Toggle` widget for us.
        let toggle = widget::Toggle::new(app.bool_matrix[col][row])
            .rgba(r, g, b, a)
            .border(app.border_width);
        if let Some(new_value) = elem.set(toggle, ui).last() {
            app.bool_matrix[col][row] = new_value;
        }
    }

    // A demonstration using a DropDownList to select its own color.
    for selected_idx in widget::DropDownList::new(&app.ddl_colors, app.selected_idx)
        .w_h(150.0, 40.0)
        .right_from(ids.slider_height, 30.0) // Position right from widget 6 by 50 pixels.
        .max_visible_items(3)
        .color(app.ddl_color)
        .border(app.border_width)
        .border_color(app.ddl_color.plain_contrast())
        .label("Colors")
        .label_color(app.ddl_color.plain_contrast())
        .scrollbar_on_top()
        .set(ids.color_select, ui)
    {
        app.selected_idx = Some(selected_idx);
        app.ddl_color = match &app.ddl_colors[selected_idx][..] {
            "Black" => color::BLACK,
            "White" => color::WHITE,
            "Red"   => color::RED,
            "Green" => color::GREEN,
            "Blue"  => color::BLUE,
            _       => color::PURPLE,
        }
    }

    // Draw an xy_pad.
    for (x, y) in widget::XYPad::new(app.circle_pos[0], -75.0, 75.0, // x range.
                                     app.circle_pos[1], 95.0, 245.0) // y range.
        .w_h(150.0, 150.0)
        .right_from(ids.toggle_matrix, 30.0)
        .align_bottom_of(ids.toggle_matrix) // Align to the bottom of the last toggle_matrix element.
        .color(app.ddl_color)
        .border(app.border_width)
        .border_color(color::WHITE)
        .label("Circle Position")
        .label_color(app.ddl_color.plain_contrast().alpha(0.5))
        .line_thickness(2.0)
        .set(ids.circle_position, ui)
    {
        app.circle_pos[0] = x;
        app.circle_pos[1] = y;
    }

    // Draw a circle at the app's circle_pos.
    widget::Circle::fill(15.0)
        .xy_relative_to(ids.circle_position, app.circle_pos)
        .color(app.ddl_color)
        .set(ids.old_circle, ui);

    // Draw two TextBox and EnvelopeEditor pairs to the right of the DropDownList flowing downward.
    for i in 0..2 {
        let &mut (ref mut env, ref mut text) = &mut app.envelopes[i];
        let (text_box, env_editor, env_y_max, env_skew_y) = match i {
            0 => (ids.text_box_a, ids.envelope_editor_a, 20_000.0, 3.0),
            1 => (ids.text_box_b, ids.envelope_editor_b, 1.0, 1.0),
            _ => unreachable!(),
        };

        // A text box in which we can mutate a single line of text, and trigger reactions via the
        // `Enter`/`Return` key.
        for event in widget::TextBox::new(text)
            .and_if(i == 0, |text| text.right_from(ids.color_select, 30.0))
            .font_size(20)
            .w_h(320.0, 40.0)
            .border(app.border_width)
            .border_color(app.bg_color.invert().plain_contrast())
            .color(app.bg_color.invert())
            .set(text_box, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("TextBox {}: {:?}", i, text),
                widget::text_box::Event::Update(string) => *text = string,
            }
        }

        // Draw an EnvelopeEditor. (&[Point], x_min, x_max, y_min, y_max).
        for event in widget::EnvelopeEditor::new(env, 0.0, 1.0, 0.0, env_y_max)
            .down(10.0)
            .w_h(320.0, 150.0)
            .skew_y(env_skew_y)
            .color(app.bg_color.invert())
            .border(app.border_width)
            .border_color(app.bg_color.invert().plain_contrast())
            .label(&text)
            .label_color(app.bg_color.invert().plain_contrast().alpha(0.5))
            .point_radius(6.0)
            .line_thickness(2.0)
            .set(env_editor, ui)
        {
            event.update(env);
        }
    }


    /////////////////////
    ///// Scrollbar /////
    /////////////////////


    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);
}

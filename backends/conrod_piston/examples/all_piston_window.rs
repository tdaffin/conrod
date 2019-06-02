//! An example demonstrating all widgets in a long, vertically scrollable window.

extern crate conrod_core;
extern crate conrod_example_shared;
extern crate find_folder;
extern crate piston_window;
extern crate conrod_piston;

use self::piston_window::{AdvancedWindow, PistonWindow, UpdateEvent, Window, WindowSettings};
use self::piston_window::{Flip, G2d, G2dTexture, Texture, TextureSettings};
use self::piston_window::OpenGL;
use self::piston_window::texture::UpdateTexture;

pub fn main() {
    let mut manager = conrod_example_shared::Manager::new();
    let namer = conrod_example_shared::Namer::new("Piston Backend");

    // Construct the window.
    let size = manager.example().info().size;
    let mut window: PistonWindow =
        WindowSettings::new(namer.title(&manager.example().info().name), [size.0, size.1])
            .opengl(OpenGL::V3_2) // If not working, try `OpenGL::V2_1`.
            .samples(4)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    manager.ui().fonts.insert_from_file(font_path).unwrap();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(size.0, size.1)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = size.0 as usize * size.1 as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init,
            size.0, size.1, &settings).unwrap();
        (cache, texture)
    };

    // Load the rust logo from file to a piston_window texture.
    let rust_logo: G2dTexture = {
        let assets = find_folder::Search::ParentsThenKids(5, 3).for_folder("assets").unwrap();
        let path = assets.join("images/rust.png");
        let factory = &mut window.factory;
        let settings = TextureSettings::new();
        Texture::from_path(factory, &path, Flip::None, &settings).unwrap()
    };

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we only have one image, however the macro may be used to list multiple.
    let mut image_map = conrod_core::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    // Instantiate the generated list of widget identifiers.
    manager.init(rust_logo);

    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the src event to a conrod event.
        let size = window.size();
        let (win_w, win_h) = (size.width as conrod_core::Scalar, size.height as conrod_core::Scalar);
        if let Some(e) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            if let Some(_) = manager.handle_event(e) {
                let w = &mut window.window;
                let info = manager.example().info();
                w.set_title(namer.title(&info.name));
                w.set_size(info.size);
            }
        }

        event.update(|_| manager.update());

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = manager.ui().draw_if_changed() {

                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod_core::text::rt::Rect<u32>,
                                           data: &[u8]|
                    {
                        let offset = [rect.min.x, rect.min.y];
                        let size = [rect.width(), rect.height()];
                        let format = piston_window::texture::Format::Rgba8;
                        let encoder = &mut graphics.encoder;
                        text_vertex_data.clear();
                        text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                        UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                            .expect("failed to update texture")
                    };

                // Specify how to get the drawable texture from the image. In this case, the image
                // *is* the texture.
                fn texture_from_image<T>(img: &T) -> &T { img }

                // Draw the conrod `render::Primitives`.
                conrod_piston::draw::primitives(primitives,
                                                context,
                                                graphics,
                                                &mut text_texture_cache,
                                                &mut glyph_cache,
                                                &image_map,
                                                cache_queued_glyphs,
                                                texture_from_image);
            }
        });
    }
}

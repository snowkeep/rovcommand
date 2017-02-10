extern crate piston_window;
extern crate find_folder;

use conrod;
use std;

use self::piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};
use self::piston_window::{Flip, G2d, G2dTexture, Texture, TextureSettings};
use self::piston_window::OpenGL;
use self::piston_window::texture::UpdateTexture;

pub struct GuiState {
    rovs: u32,
    state: u32
}

impl GuiState {
    fn new() -> Self {
        GuiState {
            rovs: 0,
            state: 0
        }
    }
}

pub fn main() {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;

    // construct the window
    let mut window: PistonWindow = 
        WindowSettings::new("ROV Command", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .samples(4)
//            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

    // construct the UI
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(theme())
        .build();

    // add a font to the ui font::map
    // TODO: pull into theme?
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // create a textrue to use for caching text on the GPU
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod::text::GlyphCache::new(WIDTH, HEIGHT, SCALE_TOLERANCE, POSITION_TOLERANCE);
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128, buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap();
        (cache, texture)
    };

    // instantiate the widget identifiers
    let ids = Ids::new(ui.widget_id_generator());

    // create conrod::image::Map
    let mut image_map = conrod::image::Map::new();

    // state struct
    let mut state = GuiState::new();

    // poll events from the window
    while let Some(event) = window.next() {

        // convert piston event to conrod event
        let size = window.size();
        let (win_w, win_h) = (size.width as conrod::Scalar, size.height as conrod::Scalar);
        if let Some(e) = conrod::backend::piston::event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            layout(&mut ui, &ids, &mut state);
        });

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {

                // cache glyphs to texture cache
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod::text::rt::Rect<u32>,
                                           data: &[u8]|
                {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size).expect("failed to update texture")
                };

                fn texture_from_image<T>(img: &T) -> &T { img }

                // draw the conrod render::Primitives
                conrod::backend::piston::draw::primitives(primitives, context, graphics, &mut text_texture_cache, &mut glyph_cache, &image_map, cache_queued_glyphs, texture_from_image);
            }
        });

    }

}

// consider loading this from a file
// and a themeing gui
fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "ROV Command Default".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: std::collections::HashMap::new(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

// generate unique 'WidgetId' for each widget
widget_ids! {
    pub struct Ids {
        canvas,
    }
}

// instantiate the gui
fn layout(ui: &mut conrod::UiCell, ids: &Ids, state: &mut GuiState) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    
    const TITLE: &'static str = "All Widgets";
    widget::Canvas::new().pad(MARGIN).set(ids.canvas, ui);
}

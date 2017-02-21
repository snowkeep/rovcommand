extern crate find_folder;
//extern crate image;

use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use std;
use std::sync::mpsc::Receiver;

//use chan::{Receiver, Sender};

const WIN_W: u32 = 1024;
const WIN_H: u32 = 768;

struct GuiState {
    log: Vec<[String; 2]>
}

impl GuiState {
    fn new() -> Self {
        GuiState {
            // vecs of [source, message]
            log: Vec::new()
        }
    }
}

pub fn main(log_rx: Receiver<[String; 2]>) {

    // build the window
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIN_W, WIN_H)
        .with_title("ROV Command")
        .build_glium()
        .unwrap();

    // construct the UI
    let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        .theme(theme())
        .build();

    // add a font to the ui font::map
    // TODO: pull into theme?
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // instantiate the widget identifiers
    let ids = Ids::new(ui.widget_id_generator());

    // create conrod::image::Map (empty
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // state struct
    let mut state = GuiState::new();

    // converter for conrod::render::Primitives to glium Surface commands
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // poll events from the window
    let mut event_loop = EventLoop::new();
    'main: loop {

        // handle all events
        for event in event_loop.next(&display) {
            // use winit backend feature to convert winit event to conrod event
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                // break from loop on ESC
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,
                _ => {},
            }
        }

        // check for new log info and if there is, append to state.log
        let new_log = log_rx.try_recv();
        if new_log.is_ok() {
            state.log.push(new_log.unwrap());
        }

        // set up the game window layout
        layout(&mut ui.set_widgets(), &ids, &mut state);

//        // instantiate gui
//        build_board_ui(ui);

        // draw the ui
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
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
        // window layout canvases
        app,       // full window
        game,       // top half - play and control
        board,      // view and Controls
        view_port,
        controls,
        status,     // ROV selection and status
        console,
        console_text
    }
}

// instantiate the gui
fn layout(ui: &mut conrod::UiCell, ids: &Ids, state: &mut GuiState) {
    use conrod::{widget, Colorable, Positionable, Widget};
    use conrod::color;


    const MARGIN: conrod::Scalar = 10.0;
//    const SHAPE_GAP: conrod::Scalar = 50.0;

    // placement
    widget::Canvas::new().flow_down(&[
        (ids.game, widget::Canvas::new().length((WIN_H * 3/4) as f64).flow_right(&[
            (ids.board, widget::Canvas::new().length((WIN_W * 5/6) as f64).flow_down(&[
                (ids.view_port, widget::Canvas::new().length((WIN_H * 11/16) as f64).color(color::BLUE)),
                (ids.controls, widget::Canvas::new())
            ])),
            (ids.status, widget::Canvas::new().color(color::LIGHT_CHARCOAL))
        ])),
        (ids.console, widget::Canvas::new().color(color::LIGHT_GREY))
    ]).set(ids.app, ui);

    // concatenate the logs together
    let mut log_text = "".to_string();
    for entry in &state.log {
        log_text = log_text + &format!("{:10}: {}\n", entry[0], entry[1]);
    }

    widget::Text::new(&log_text)
        .color(color::BLACK)
        .top_left_with_margin_on(ids.console, MARGIN)
        .set(ids.console_text, ui);

}

struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant
}

impl EventLoop {
    fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true
        }
    }

    fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // no need to be faster than 60 FPS, so wait at least 16ms
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(15);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // collect all pending events
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // if there are no events and ui does not need updating, wait
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    // notifies event loop that the ui needs another update
    // primarily used when parts of the ui are still animating
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use gui::*;
    use std::{thread, time};
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};

    use self::rand::Rng;

    #[test]
    fn send_to_gui() {
        let (log_tx, log_rx) = mpsc::channel();

        let ui = thread::spawn(|| {
            main(log_rx);
        });

        let delay_s = rand::thread_rng().gen_range(1, 10);
        for x in 0..10 {
           println!("pass {}", x);
            let delay = time::Duration::new(delay_s, 0);
            thread::sleep(delay);
            let msg = format!("test log message {}", x);
            log_tx.send(["test".to_string(), msg]);
        }

        let _ = ui.join();

    }

}

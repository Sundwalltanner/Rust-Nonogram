use chrono::{DateTime, Duration, Utc};
use glutin_window::GlutinWindow;
use graphics::color::hex;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub use nonogram_board::NonogramBoard;
pub use nonogram_board_view::{NonogramView, NonogramViewSettings};
pub use nonogram_controller::NonogramController;

mod nonogram_board;
mod nonogram_board_view;
mod nonogram_controller;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Nonogram", [925, 875])
        .graphics_api(opengl)
        .samples(4)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);
    let mut nonogram = NonogramBoard::new();
    nonogram.initialize();
    let mut nonogram_controller = NonogramController::new(nonogram);
    let mut nonogram_view_settings =
        NonogramViewSettings::new(nonogram_controller.nonogram.dimensions);
    let nonogram_view = NonogramView::new(nonogram_view_settings);

    use piston::AdvancedWindow;
    use piston::Window;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font = &assets.join("FiraSans-Regular.ttf");
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = &mut GlyphCache::new(font, (), texture_settings).expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        nonogram_controller.event(
            nonogram_view.settings.position,
            nonogram_view.settings.board_dimensions,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                let dur = match nonogram_controller.nonogram.game_start {
                    Some(game_start) => match nonogram_controller.nonogram.game_end {
                        Some(game_end) => game_end - game_start,
                        None => Utc::now() - game_start,
                    },
                    None => Duration::seconds(0),
                };
                clear(hex("222222"), g);
                nonogram_view.draw(
                    &nonogram_controller,
                    glyphs,
                    &c,
                    g,
                    dur,
                    nonogram_controller.nonogram.count_black,
                    nonogram_controller.nonogram.goal_black,
                );
            });
        }
        if let Some(end) = nonogram_controller.nonogram.game_end {
            nonogram_controller.nonogram = nonogram_board::NonogramBoard::new();
            nonogram_controller.nonogram.initialize();
        }
    }
}

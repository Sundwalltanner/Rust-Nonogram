use std::time::{Instant, Duration};
use std::fs;
use serde_json::{Result, Value};
use glutin_window::GlutinWindow;
use graphics::color::hex;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::Window;
use piston::window::WindowSettings;

pub use nonogram_board::NonogramBoard;
pub use nonogram_board_view::{NonogramView, NonogramViewSettings};
pub use nonogram_controller::NonogramController;

mod common;
mod nonogram_board;
mod nonogram_board_view;
mod nonogram_controller;

use crate::common::{INITIAL_WINDOW_SIZE, INITIAL_BOARD_DIMENSIONS};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Nonogram", INITIAL_WINDOW_SIZE)
        .graphics_api(opengl)
        .samples(4);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);
    let mut nonogram = NonogramBoard::new(INITIAL_BOARD_DIMENSIONS, false);
    let mut nonogram_controller = NonogramController::new(nonogram);
    let mut nonogram_view_settings =
        NonogramViewSettings::new(nonogram_controller.nonogram.dimensions);
    let mut nonogram_view = NonogramView::new(nonogram_view_settings);

    use piston::AdvancedWindow;
    use piston::Window;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let font = &assets.join("FiraSans-Regular.ttf");
    let glyphs = &mut GlyphCache::new(font, (), texture_settings)
        .expect("Could not load FiraSans-Regular.ttf");
    let mark_font = &assets.join("Monoround.ttf");
    let mark_glyphs = &mut GlyphCache::new(mark_font, (), texture_settings)
        .expect("Could not load Monoround.ttf");
    let material_icons_font = &assets.join("MaterialIcons-Regular.ttf");
    let material_icons_glyphs = &mut GlyphCache::new(material_icons_font, (), texture_settings)
        .expect("Could not load MaterialIcons-Regular.ttf");

    while let Some(e) = events.next(&mut window) {
        nonogram_controller.event(
            nonogram_view.settings.position,
            nonogram_view.settings.board_dimensions,
            nonogram_view.settings.dimensions_dropdown_menu_box,
            nonogram_view.settings.restart_box,
            nonogram_view.settings.new_game_box,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                if !nonogram_controller.nonogram.end_game_screen {
                    nonogram_controller.nonogram.duration = match nonogram_controller.nonogram.game_start {
                        Some(game_start) => match nonogram_controller.nonogram.game_end {
                            Some(game_end) => game_end - game_start,
                            None => Instant::now() - game_start,
                        },
                        None => Duration::from_secs(0),
                    };
                }
                clear(hex("222222"), g);
                nonogram_view.draw(
                    &nonogram_controller,
                    glyphs,
                    mark_glyphs,
                    material_icons_glyphs,
                    &c,
                    g,
                    nonogram_controller.nonogram.count_black,
                    nonogram_controller.nonogram.goal_black,
                    window.size(),
                );
            });
        }
        if nonogram_controller.nonogram.reset_board {
            nonogram_controller.nonogram = nonogram_board::NonogramBoard::new(nonogram_controller.nonogram.next_dimensions, true);
            nonogram_view_settings = NonogramViewSettings::new(nonogram_controller.nonogram.dimensions);
            nonogram_view = NonogramView::new(nonogram_view_settings);
        }
    }
}

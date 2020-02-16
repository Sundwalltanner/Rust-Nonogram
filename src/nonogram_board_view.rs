use chrono::Duration;
use graphics::character::CharacterCache;
use graphics::color::hex;
use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::NonogramController;

#[derive(Default)]
/// Stores nonogram view settings.
pub struct NonogramViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub board_dimensions: [f64; 2],
    pub cell_dimensions: [usize; 2],
    pub cell_size: f64,
    pub background_color: Color,
    pub border_color: Color,
    pub board_edge_color: Color,
    pub section_edge_color: Color,
    pub cell_edge_color: Color,
    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,
    pub selected_cell_border_color: Color,
    pub selected_cell_border_round_radius: f64,
    pub selected_cell_border_radius: f64,
    pub filled_cell_background_color: Color,
    pub marked_cell_background_color: Color,
    pub text_color: Color,
}

impl NonogramViewSettings {
    /// Creates new nonogram view settings.
    pub fn new(new_cell_dimensions: [usize; 2]) -> NonogramViewSettings {
        let mut view_settings = NonogramViewSettings {
            position: [300.0, 240.0],
            size: 1000.0,
            board_dimensions: [0.0; 2],
            cell_dimensions: [new_cell_dimensions[0], new_cell_dimensions[1]],
            cell_size: 0.0,
            background_color: hex("f7f5f6"),
            border_color: hex("cccccc"),
            board_edge_color: hex("cccccc"),
            section_edge_color: hex("34af4a"),
            cell_edge_color: hex("cccccc"),
            board_edge_radius: 2.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 2.0,
            selected_cell_border_color: hex("5adbfd"),
            selected_cell_border_round_radius: 2.0,
            selected_cell_border_radius: 2.0,
            filled_cell_background_color: hex("353235"),
            marked_cell_background_color: hex("f77b00"),
            text_color: hex("ffffff"),
        };
        view_settings.calc_dimensions();
        view_settings
    }

    fn calc_dimensions(&mut self) {
        let cols = self.cell_dimensions[0] as f64;
        let rows = self.cell_dimensions[1] as f64;
        self.board_dimensions[0] = (cols / (cols + rows)) * self.size;
        self.board_dimensions[1] = (rows / (cols + rows)) * self.size;
        self.cell_size = self.board_dimensions[0] / cols;
    }
}

/// Stores visual information about a nonogram.
pub struct NonogramView {
    /// Stores nonogram view settings.
    pub settings: NonogramViewSettings,
}

impl NonogramView {
    /// Creates a new nonogram view.
    pub fn new(settings: NonogramViewSettings) -> NonogramView {
        NonogramView { settings }
    }

    /// Draw nonogram.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &NonogramController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
        duration: Duration,
        count_black: u64,
        goal_black: u64,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        use graphics::text::Text;
        use graphics::{Line, Rectangle, Transformed};

        let settings = &self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.board_dimensions[0],
            settings.board_dimensions[1],
        ];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw filled cell background.
        for j in 0..settings.cell_dimensions[0] {
            for i in 0..settings.cell_dimensions[1] {
                let value = controller.nonogram.get([j, i]);
                if value == 1 {
                    let pos = [j as f64 * settings.cell_size, i as f64 * settings.cell_size];
                    let cell_rect = [
                        settings.position[0] + pos[0],
                        settings.position[1] + pos[1],
                        settings.cell_size,
                        settings.cell_size,
                    ];
                    Rectangle::new(settings.filled_cell_background_color).draw(
                        cell_rect,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                } else if value == 2 {
                    let text = Text::new_color(settings.marked_cell_background_color, 40);
                    let ch_x = settings.position[0] + j as f64 * settings.cell_size + 10.0;
                    let ch_y = settings.position[1] + i as f64 * settings.cell_size + 30.0;
                    text.draw("x", glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                        .unwrap_or_else(|_| panic!("text draw failed"));
                }
            }
        }

        // Draw column and row hint numbers.
        let hint_reg = Text::new_color(hex("ffffff"), 15);
        let hint_cross = Text::new_color(hex("666666"), 15);
        let mut ch_x = 0.0;
        let mut ch_y = 0.0;

        // Draw column numbers.
        for k in 0..controller.nonogram.dimensions[0] as usize {
            let mut num_pos = 0;
            for i in 0..controller.nonogram.nums_per[0] as usize {
                let hint_val = controller.nonogram.goal_nums[0][k][i];
                let ch = hint_val.abs().to_string();
                if ch != "0" {
                    ch_x = settings.position[1] + k as f64 * settings.cell_size + 75.0;
                    ch_y = settings.position[0] - num_pos as f64 * 20.0 - 80.0;
                    if hint_val > 0 {
                        hint_reg.draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                            .unwrap_or_else(|_| panic!("text draw failed"));
                    } else {
                        hint_cross.draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                            .unwrap_or_else(|_| panic!("text draw failed"));
                    }
                    num_pos += 1;
                }
            }
        }

        // Draw row numbers.
        for k in 0..controller.nonogram.dimensions[1] as usize {
            let mut num_pos = 0;
            for i in 0..controller.nonogram.nums_per[1] as usize {
                let hint_val = controller.nonogram.goal_nums[1][k][i];
                let ch = hint_val.abs().to_string();
                if ch != "0" {
                    ch_x = settings.position[0] - num_pos as f64 * 20.0 - 25.0;
                    ch_y = settings.position[1] + k as f64 * settings.cell_size + 30.0;
                    if hint_val > 0 {
                        hint_reg.draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                            .unwrap_or_else(|_| panic!("text draw failed"));
                    } else {
                        hint_cross.draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                            .unwrap_or_else(|_| panic!("text draw failed"));
                    }
                    num_pos += 1;
                }
            }
        }

        // Draw cell borders.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..controller.nonogram.dimensions[0] {
            // Skip lines that are covered by sections.
            if (i % 5) == 0 {
                continue;
            }

            let x = settings.position[0]
                + i as f64 / controller.nonogram.dimensions[0] as f64
                    * settings.cell_size
                    * controller.nonogram.dimensions[0] as f64;
            let y2 = settings.position[1]
                + settings.cell_size * controller.nonogram.dimensions[1] as f64;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);
        }
        for i in 0..controller.nonogram.dimensions[1] {
            // Skip lines that are covered by sections.
            if (i % 5) == 0 {
                continue;
            }

            let y = settings.position[1]
                + i as f64 / controller.nonogram.dimensions[1] as f64
                    * settings.cell_size
                    * controller.nonogram.dimensions[1] as f64;
            let x2 = settings.position[0]
                + settings.cell_size * controller.nonogram.dimensions[0] as f64;

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw section borders.
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 1..(controller.nonogram.dimensions[0] / 5) {
            // Set up coordinates.
            let x = settings.position[0]
                + i as f64 / (controller.nonogram.dimensions[0] / 5) as f64
                    * settings.cell_size
                    * controller.nonogram.dimensions[0] as f64;
            let y2 = settings.position[1]
                + settings.cell_size * controller.nonogram.dimensions[1] as f64;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);
        }
        for i in 1..(controller.nonogram.dimensions[1] / 5) {
            // Set up coordinates.
            let y = settings.position[1]
                + i as f64 / (controller.nonogram.dimensions[1] / 5) as f64
                    * settings.cell_size
                    * controller.nonogram.dimensions[1] as f64;
            let x2 = settings.position[0]
                + settings.cell_size * controller.nonogram.dimensions[0] as f64;

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        // Draw info box.
        let info_box_rect = [20.0, 70.0, 250.0, 150.0];

        Rectangle::new_round(hex("333333"), 10.0).draw(
            info_box_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        // Draw nonogram title.
        Text::new_color(hex("ffffff"), 25)
            .draw(
                &"NONOGRAM",
                glyphs,
                &c.draw_state,
                c.transform.trans(80.0, 60.0),
                g,
            )
            .unwrap_or_else(|_| panic!("text draw failed"));

        // Draw progress title.
        Text::new_color(hex("ffffff"), 12)
            .draw(
                &"PROGRESS",
                glyphs,
                &c.draw_state,
                c.transform.trans(115.0, 95.0),
                g,
            )
            .unwrap_or_else(|_| panic!("text draw failed"));

        // Draw progress.
        Text::new_color(hex("ffffff"), 25)
            .draw(
                &*format!(
                    "{} / {} ({:.2}%)",
                    count_black,
                    goal_black,
                    (count_black as f32 / goal_black as f32) * 100.0
                ),
                glyphs,
                &c.draw_state,
                c.transform.trans(70.0, 120.0),
                g,
            )
            .unwrap_or_else(|_| panic!("text draw failed"));

        let total_seconds = duration.num_seconds();
        let total_mins = total_seconds / 60;
        let total_hrs = total_mins / 60;
        let rem_seconds = total_seconds - total_mins * 60;
        let rem_mins = total_mins - total_hrs * 60;

        // Draw timer title.
        Text::new_color(hex("ffffff"), 12)
            .draw(
                &"TIMER",
                glyphs,
                &c.draw_state,
                c.transform.trans(125.0, 160.0),
                g,
            )
            .unwrap_or_else(|_| panic!("text draw failed"));

        // Draw timer.
        Text::new_color(hex("ffffff"), 50)
            .draw(
                &*format!("{:02}:{:02}:{:02}", total_hrs, rem_mins, rem_seconds),
                glyphs,
                &c.draw_state,
                c.transform.trans(50.0, 200.0),
                g,
            )
            .unwrap_or_else(|_| panic!("text draw failed"));

        // Draw selected cell border.
        if let Some(ind) = controller.selected_cell {
            let pos = [
                ind[0] as f64 * settings.cell_size,
                ind[1] as f64 * settings.cell_size,
            ];
            let cell_rect = [
                settings.position[0] + pos[0],
                settings.position[1] + pos[1],
                settings.cell_size,
                settings.cell_size,
            ];
            Rectangle::new_round_border(
                settings.selected_cell_border_color,
                settings.selected_cell_border_round_radius,
                settings.selected_cell_border_radius,
            )
            .draw(cell_rect, &c.draw_state, c.transform, g);
        }
    }
}

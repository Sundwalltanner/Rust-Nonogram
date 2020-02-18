use chrono::Duration;
use graphics::character::CharacterCache;
use graphics::color::hex;
use graphics::types::Color;
use graphics::{Context, Graphics};
use piston::window::Size;

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
        mark_glyphs: &mut C,
        c: &Context,
        g: &mut G,
        duration: Duration,
        count_black: u64,
        goal_black: u64,
        window_size: Size,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        use graphics::text::Text;
        use graphics::{Line, Rectangle, Transformed};

        let settings = &self.settings;

        // Rectangle the size of the entire inner window.
        let window_rect = [
            0.0,
            0.0,
            window_size.width,
            window_size.height,
        ];

        let total_seconds = duration.num_seconds();
        let total_mins = total_seconds / 60;
        let total_hrs = total_mins / 60;
        let rem_seconds = total_seconds - total_mins * 60;
        let rem_mins = total_mins - total_hrs * 60;

        // Draw a large transparent rectangle over window.
        // Last two digits in hex refer to transparency: https://css-tricks.com/8-digit-hex-codes/
        // Rectangle::new(hex("000000E6")).draw(window_rect, &c.draw_state, c.transform, g);

        // Draw win screen.
        if let Some(ind) = controller.nonogram.game_end {
        //if true {
            let mut win_box_rect = [
                window_size.width / 2.0,
                window_size.height / 2.0,
                300.0,
                500.0,
            ];

            win_box_rect[0] = win_box_rect[0] - (win_box_rect[2] / 2.0);
            win_box_rect[1] = win_box_rect[1] - (win_box_rect[3] / 2.0);
            //println!("Width: {}, Height: {}", win_box_rect[0], win_box_rect[1]);

            Rectangle::new_round(hex("333333"), 10.0).draw(win_box_rect, &c.draw_state, c.transform, g);

            // Draw win title.
            let win_title_str = "YOU WON".to_string();
            let win_title_size = 25;
            let win_title_width = match glyphs.width(win_title_size, &win_title_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let win_title_loc = [
                win_box_rect[0] + (win_box_rect[2] / 2.0) - (win_title_width / 2.0),
                180.0
            ];
            Text::new_color(hex("ffffff"), win_title_size)
                .draw(
                    &win_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(win_title_loc[0], win_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            let mut stat_row_y = 250.0;
            let stat_row_margins = [30.0, 30.0];
            let stat_row_x = [win_box_rect[2] + stat_row_margins[0], win_box_rect[2] + win_box_rect[0] - stat_row_margins[0]];

            // Left-aligned timer title.
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &"TIME",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[0], stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Right-aligned stat indicating what the timer ended on when previous puzzle was solved.
            let timer_str = format!("{:02}:{:02}:{:02}", total_hrs, rem_mins, rem_seconds);
            let timer_size = 25;
            let timer_width = match glyphs.width(timer_size, &timer_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &format!("{}", timer_str),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[1] - timer_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Left-aligned black count title.
            stat_row_y += stat_row_margins[1];
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &"BLACK",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[0], stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Right-aligned count of black/filled squares.
            let black_count_str = format!("{:>8}", controller.nonogram.goal_black);
            let black_count_size = 25;
            let black_count_width = match glyphs.width(black_count_size, &black_count_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &format!("{}", black_count_str),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[1] - black_count_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // New stat row.
            stat_row_y += stat_row_margins[1];

            // Left-aligned total square count title.
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &"TOTAL",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[0], stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Right-aligned total count of squares.
            let total_count = controller.nonogram.dimensions[0] * controller.nonogram.dimensions[1];
            let total_count_str = format!("{}", total_count);
            let total_count_size = 25;
            let total_count_width = match glyphs.width(total_count_size, &total_count_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &format!("{}", total_count_str),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[1] - total_count_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // New stat row.
            stat_row_y += stat_row_margins[1];

            // Left-aligned black_square/total_square ratio title.
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &"RATIO",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[0], stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Right-aligned black_square/total_square ratio title.
            let black_total_ratio = controller.nonogram.goal_black as f64 / total_count as f64;
            let black_total_ratio_str = format!("{:.2}", black_total_ratio);
            let black_total_ratio_size = 25;
            let black_total_ratio_width = match glyphs.width(black_total_ratio_size, &black_total_ratio_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &format!("{}", black_total_ratio_str),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[1] - black_total_ratio_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // New stat row.
            stat_row_y += stat_row_margins[1];

            // Left-aligned dimensions title.
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &"DIMENSIONS",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[0], stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Right-aligned dimensions.
            let dimensions_str = format!("{}x{}", controller.nonogram.dimensions[0], controller.nonogram.dimensions[1]);
            let dimensions_size = 25;
            let dimensions_width = match glyphs.width(dimensions_size, &dimensions_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            Text::new_color(hex("ffffff"), 25)
                .draw(
                    &format!("{}", dimensions_str),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(stat_row_x[1] - dimensions_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

        } else {

            let board_rect = [
                settings.position[0],
                settings.position[1],
                settings.board_dimensions[0],
                settings.board_dimensions[1],
            ];

            // Draw board background.
            Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

            // Draw filled cell background.
            // We calculate the height of text by multiplying font size by 0.75 in order to convert between pixels and points.
            let mark_size = (settings.cell_size / 1.5) as u32;
            let mark_width = match mark_glyphs.width(mark_size, &"x") {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let mark_loc = [
                (settings.cell_size / 2.0) - (mark_width as f64 / 2.0),
                (settings.cell_size / 2.0) + ((mark_size as f64 * 0.75) / 2.0),
            ];
            let mark_text = Text::new_color(settings.marked_cell_background_color, mark_size);

            for col in 0..settings.cell_dimensions[0] {
                for row in 0..settings.cell_dimensions[1] {
                    let value = controller.nonogram.get([col, row]);
                    let pos = [col as f64 * settings.cell_size, row as f64 * settings.cell_size];
                    if value == 1 {
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
                        mark_text.draw(
                            "x",
                            mark_glyphs,
                            &c.draw_state,
                            c.transform.trans(
                                settings.position[0] + pos[0] + mark_loc[0],
                                settings.position[1] + pos[1] + mark_loc[1],
                            ),
                            g,
                        )
                        .unwrap_or_else(|_| panic!("text draw failed"));
                        
                    }
                }
            }

            // Draw column and row hint numbers.
            // We calculate the height of text by multiplying font size by 0.75 in order to convert between pixels and points.
            let hint_num_size = 15;
            let hint_reg = Text::new_color(hex("ffffff"), hint_num_size);
            let hint_cross = Text::new_color(hex("666666"), hint_num_size);
            let mut ch_x = 0.0;
            let mut ch_y = 0.0;

            // Draw column hint numbers.
            // Currently this logic goes through the effort of finding the width of each individual number
            // in order to try and center all the numbers in a column. This might not be worth the effort,
            // as it's only really noticeable when the numbers start hitting the double digits.
            for k in 0..settings.cell_dimensions[0] as usize {
                let mut num_pos = 0;
                for i in 0..controller.nonogram.nums_per[0] as usize {
                    let hint_val = controller.nonogram.goal_nums[0][k][i];

                    // Only draw column numbers that aren't 0.
                    if hint_val != 0 {
                        let ch = hint_val.abs().to_string();
                        let hint_num_width = match glyphs.width(hint_num_size, &ch) {
                            Ok(v) => v,
                            Err(e) => 0.0,
                        };
                        let col_num_loc = (settings.cell_size / 2.0) - (hint_num_width as f64 / 2.0);
                        ch_x = settings.position[0] + (k as f64 * settings.cell_size) + col_num_loc;
                        ch_y = settings.position[0] - num_pos as f64 * 20.0 - 80.0;

                        // Either draw a normal number, or draw a crossout number.
                        if hint_val > 0 {
                            hint_reg
                                .draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                                .unwrap_or_else(|_| panic!("text draw failed"));
                        } else {
                            hint_cross
                                .draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                                .unwrap_or_else(|_| panic!("text draw failed"));
                        }
                        num_pos += 1;
                    }
                }
            }

            // Draw row hint numbers.
            let row_num_loc = (settings.cell_size / 2.0) + ((hint_num_size as f64 * 0.75) / 2.0);
            for k in 0..settings.cell_dimensions[1] as usize {
                let mut num_pos = 0;
                for i in 0..controller.nonogram.nums_per[1] as usize {
                    let hint_val = controller.nonogram.goal_nums[1][k][i];

                    // Only draw row numbers that aren't 0.
                    if hint_val != 0 {
                        let ch = hint_val.abs().to_string();
                        ch_x = settings.position[0] - num_pos as f64 * 20.0 - 25.0;
                        ch_y = settings.position[1] + (k as f64 * settings.cell_size) + row_num_loc;

                        // Either draw a normal number, or draw a crossout number.
                        if hint_val > 0 {
                            hint_reg
                                .draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
                                .unwrap_or_else(|_| panic!("text draw failed"));
                        } else {
                            hint_cross
                                .draw(&ch, glyphs, &c.draw_state, c.transform.trans(ch_x, ch_y), g)
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
            let nonogram_title_str = "NONOGRAM".to_string();
            let nonogram_title_size = 25;
            let nonogram_title_width = match glyphs.width(nonogram_title_size, &nonogram_title_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let nonogram_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (nonogram_title_width / 2.0),
                60.0
            ];
            Text::new_color(hex("ffffff"), nonogram_title_size)
                .draw(
                    &nonogram_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(nonogram_title_loc[0], nonogram_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw progress title.
            let progress_title_str = "PROGRESS".to_string();
            let progress_title_size = 12;
            let progress_title_width = match glyphs.width(progress_title_size, &progress_title_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let progress_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (progress_title_width / 2.0),
                95.0
            ];
            Text::new_color(hex("ffffff"), progress_title_size)
                .draw(
                    &progress_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(progress_title_loc[0], progress_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw progress.
            let progress_str = format!("{} / {} ({:.2}%)", count_black, goal_black, (count_black as f32 / goal_black as f32) * 100.0);
            let progress_size = 25;
            let progress_width = match glyphs.width(progress_size, &progress_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let progress_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (progress_width / 2.0),
                120.0
            ];
            Text::new_color(hex("ffffff"), progress_size)
                .draw(
                    &progress_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(progress_loc[0], progress_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw timer title.
            let timer_title_str = "TIMER".to_string();
            let timer_title_size = 12;
            let timer_title_width = match glyphs.width(timer_title_size, &timer_title_str) {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let timer_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (timer_title_width / 2.0),
                160.0
            ];
            Text::new_color(hex("ffffff"), timer_title_size)
                .draw(
                    &timer_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(timer_title_loc[0], timer_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw timer.
            let timer_str = format!("{:02}:{:02}:{:02}", total_hrs, rem_mins, rem_seconds);
            let timer_size = 50;
            
            // Unlike with the other drawn text, we don't use the actual string here,
            // because we don't want it to keep changing its location subtly for every
            // second that passes.
            let timer_width = match glyphs.width(timer_size, &"00:00:00") {
                Ok(v) => v,
                Err(e) => 0.0,
            };
            let timer_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (timer_width / 2.0),
                200.0
            ];
            Text::new_color(hex("ffffff"), timer_size)
                .draw(
                    &timer_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(timer_loc[0], timer_loc[1]),
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
}

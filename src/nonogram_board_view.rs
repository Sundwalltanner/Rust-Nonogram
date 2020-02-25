use graphics::character::CharacterCache;
use graphics::color::hex;
use graphics::types::Color;
use graphics::{Context, Graphics};
use piston::window::Size;
use rand::seq::SliceRandom;

use crate::common::{BOARD_SIZE, DIMENSIONS_CHOICES, IMAGE_PRE, IMAGE_NAMES, ButtonInteraction};
use crate::NonogramController;

#[derive(Default)]
/// Stores nonogram view settings.
pub struct NonogramViewSettings {
    /// X and Y coordinates of nonogram board relative to top left corner of the window.
    pub position: [f64; 2],
    /// Overall size value of nonogram board. This ends up being used in an equation which determines
    /// the width and height of the board based on how many rows and columns it has.
    pub size: f64,
    /// [width, height] of nonogram board.
    pub board_dimensions: [f64; 2],
    /// [columns, rows] in nonogram board.
    pub cell_dimensions: [usize; 2],
    /// Both the width and height of a single square in the nonogram board.
    pub cell_size: f64,
    /// [width, height] of nonogram board when displayed during win screen.
    pub win_board_dimensions: [f64; 2],
    /// Both the width and height of a single square in the nonogram board displayed during win screen.
    pub win_cell_size: f64,
    /// Nonogram board color. Determines color of unfilled square in nonogram board.
    pub background_color: Color,
    /// Color of overall nonogram board edge.
    pub board_edge_color: Color,
    /// Color of edges separating every 5 squares in nonogram board.
    pub section_edge_color: Color,
    /// Color of individual nonogram board square edge.
    pub cell_edge_color: Color,
    /// Thickness of nonogram board edge.
    pub board_edge_radius: f64,
    /// Thickness of edges separating every 5 squares in nonogram board.
    pub section_edge_radius: f64,
    /// Thickness of edges of each individual board square.
    pub cell_edge_radius: f64,
    pub selected_cell_border_color: Color,
    pub selected_cell_border_round_radius: f64,
    pub selected_cell_border_radius: f64,
    pub filled_cell_background_color: Color,
    pub marked_cell_background_color: Color,
    pub text_color: Color,
    pub dimensions_dropdown_menu_box: [f64; 4],
    pub dimensions_dropdown_menu_select_background: [f64; 4],
    pub win_box_rect: [f64; 4],
    pub restart_box: [f64; 4],
    pub new_game_box: [f64; 4],
    pub win_critique: String,
}

impl NonogramViewSettings {
    /// Creates new nonogram view settings.
    pub fn new(new_cell_dimensions: [usize; 2]) -> NonogramViewSettings {
        let mut view_settings = NonogramViewSettings {
            position: [300.0, 240.0],
            size: BOARD_SIZE,
            board_dimensions: [0.0; 2],
            cell_dimensions: [new_cell_dimensions[0], new_cell_dimensions[1]],
            cell_size: 0.0,
            win_board_dimensions: [0.0, 240.0],
            win_cell_size: 0.0,
            background_color: hex("f7f5f6"),
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
            dimensions_dropdown_menu_box: [300.0, 10.0, 100.0, 30.0],
            dimensions_dropdown_menu_select_background: [0.0; 4],
            win_box_rect: [600.0, 500.0, 250.0, 200.0],
            restart_box: [450.0, 10.0, 100.0, 30.0],
            new_game_box: [450.0, 10.0, 100.0, 30.0],
            win_critique: "".to_string(),
        };
        view_settings.init_new();
        view_settings
    }
    
    fn init_new(&mut self) {
        // Because the dimensions of the board can vary, we need to initialize the locations of cells based on these dimensions
        // and the size of the board which is set by the BOARD_SIZE const in common.rs.
        let cols = self.cell_dimensions[0] as f64;
        let rows = self.cell_dimensions[1] as f64;
        self.board_dimensions[0] = (cols / (cols + rows)) * self.size;
        self.board_dimensions[1] = (rows / (cols + rows)) * self.size;
        self.cell_size = self.board_dimensions[0] / cols;

        // A random string that's displayed near an image of the final board upon winning.
        // Ends up saying something like, "That looks just like Abraham Lincoln!".
        //
        // Makes fun of the fact that my version of the nonogram board game uses randomly generated
        // boards rather than ones that actually form images of things.
        if let Some(critique_1) = IMAGE_PRE.choose(&mut rand::thread_rng()) {
            if let Some(critique_2) = IMAGE_NAMES.choose(&mut rand::thread_rng()) {
                self.win_critique = format!("{} {}{}", critique_1[0], critique_2, critique_1[1]);
            }
        }

        // The board size when it's displayed during the end game screen needs to be a certain height in order to not end up
        // overlapping the stats box. Everything else from the width of the board to the size of the cells needs to be based around
        // this maximum height.
        self.win_cell_size = self.win_board_dimensions[1] / rows;
        self.win_board_dimensions[0] = self.win_cell_size * cols;

        // Win box containing stats is center-aligned.
        self.win_box_rect[0] = self.win_box_rect[0] - (self.win_box_rect[2] / 2.0);
        self.win_box_rect[1] = self.win_box_rect[1] - (self.win_box_rect[3] / 2.0);

        // New game box / button at the bottom of the win box is center aligned and located at the very bottom of the win box.
        self.new_game_box[2] = self.win_box_rect[2];
        self.new_game_box[0] = self.win_box_rect[0] + (self.win_box_rect[2] / 2.0) - (self.new_game_box[2] / 2.0);
        self.new_game_box[1] = self.win_box_rect[1] + self.win_box_rect[3] - self.new_game_box[3];

        // Setup dimensions dropdown menu stuff.
        self.dimensions_dropdown_menu_select_background = self.dimensions_dropdown_menu_box;
        self.dimensions_dropdown_menu_select_background[3] *= (DIMENSIONS_CHOICES.len() + 3) as f64;
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
        material_icons_glyphs: &mut C,
        c: &Context,
        g: &mut G,
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
        let window_rect = [0.0, 0.0, window_size.width, window_size.height];

        let total_seconds = controller.nonogram.duration.as_secs();
        let total_mins = total_seconds / 60;
        let total_hrs = total_mins / 60;
        let rem_seconds = total_seconds - total_mins * 60;
        let rem_mins = total_mins - total_hrs * 60;

        // Draw a large transparent rectangle over window.
        // Last two digits in hex refer to transparency: https://css-tricks.com/8-digit-hex-codes/
        // Rectangle::new(hex("000000E6")).draw(window_rect, &c.draw_state, c.transform, g);

        // Draw win screen.
        if controller.nonogram.end_game_screen {
        //if true {
            Rectangle::new_round(hex("333333"), 10.0).draw(
                settings.win_box_rect,
                &c.draw_state,
                c.transform,
                g,
            );
            // Randomly generated artist critique of player's winning image.
            let critique_size = 25;
            let critique_width = glyphs.width(critique_size, &settings.win_critique).unwrap_or(0.0);
            let critique_loc = [
                settings.win_box_rect[0] + (settings.win_box_rect[2] / 2.0) - (critique_width / 2.0),
                settings.win_box_rect[1] - 30.0,
            ];
            Text::new_color(hex("ffffff"), critique_size)
                .draw(
                    &settings.win_critique,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(critique_loc[0], critique_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            let mut stat_row_y = settings.win_box_rect[1] + 30.0;
            let stat_row_margins = [10.0, 30.0];
            let stat_row_x = [
                settings.win_box_rect[0] + stat_row_margins[0],
                settings.win_box_rect[2] + settings.win_box_rect[0] - stat_row_margins[0],
            ];

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
            let timer_width = glyphs.width(timer_size, &timer_str).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), timer_size)
                .draw(
                    &timer_str,
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
            let black_count_width = glyphs.width(black_count_size, &black_count_str).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), black_count_size)
                .draw(
                    &black_count_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(stat_row_x[1] - black_count_width, stat_row_y),
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
            let total_count_width = glyphs.width(total_count_size, &total_count_str).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), total_count_size)
                .draw(
                    &total_count_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(stat_row_x[1] - total_count_width, stat_row_y),
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
            let black_total_ratio_width = glyphs.width(black_total_ratio_size, &black_total_ratio_str).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), black_total_ratio_size)
                .draw(
                    &black_total_ratio_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(stat_row_x[1] - black_total_ratio_width, stat_row_y),
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
            let dimensions_str = format!(
                "{}x{}",
                controller.nonogram.dimensions[0], controller.nonogram.dimensions[1]
            );
            let dimensions_size = 25;
            let dimensions_width = glyphs.width(dimensions_size, &dimensions_str).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), dimensions_size)
                .draw(
                    &dimensions_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(stat_row_x[1] - dimensions_width, stat_row_y),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // New game button.
            match controller.new_game_button {
                ButtonInteraction::None => {
                    Rectangle::new_round(hex("9e4c41"), 5.0).draw(
                        settings.new_game_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Hover => {
                    Rectangle::new_round(hex("773931"), 5.0).draw(
                        settings.new_game_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Select => {
                    Rectangle::new_round(hex("633029"), 5.0).draw(
                        settings.new_game_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }

            // New game button text.
            let new_game_button_str = "NEW GAME".to_string();
            let new_game_button_size = 25;
            let new_game_button_width = glyphs.width(new_game_button_size, &new_game_button_str).unwrap_or(0.0);
            let new_game_button_loc = [
                settings.new_game_box[0] + (settings.new_game_box[2] / 2.0) - (new_game_button_width / 2.0),
                settings.new_game_box[1] + (settings.new_game_box[3] / 2.0) + ((new_game_button_size as f64 * 0.75) / 2.0)
            ];
            Text::new_color(hex("ffffff"), new_game_button_size)
                .draw(
                    &new_game_button_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(new_game_button_loc[0], new_game_button_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw board background.
            let mut board_rect = [
                settings.win_box_rect[0],
                settings.win_box_rect[1] - 300.0,
                settings.win_board_dimensions[0],
                settings.win_board_dimensions[1],
            ];

            board_rect[0] += (settings.win_box_rect[2] / 2.0) - (board_rect[2] / 2.0);
            
            Rectangle::new(settings.background_color).draw(
                board_rect,
                &c.draw_state,
                c.transform,
                g,
            );

            // Draw the game winning image.
            for col in 0..settings.cell_dimensions[0] {
                for row in 0..settings.cell_dimensions[1] {
                    let value = controller.nonogram.get([col, row]);
                    let pos = [
                        col as f64 * settings.win_cell_size,
                        row as f64 * settings.win_cell_size,
                    ];
                    if value == 1 {
                        let cell_rect = [
                            board_rect[0] + pos[0],
                            board_rect[1] + pos[1],
                            settings.win_cell_size,
                            settings.win_cell_size,
                        ];
                        Rectangle::new(settings.filled_cell_background_color).draw(
                            cell_rect,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }
            }
        } else {
            let board_rect = [
                settings.position[0],
                settings.position[1],
                settings.board_dimensions[0],
                settings.board_dimensions[1],
            ];

            // Draw board background.
            Rectangle::new(settings.background_color).draw(
                board_rect,
                &c.draw_state,
                c.transform,
                g,
            );

            // Draw filled cell background.
            // We calculate the height of text by multiplying font size by 0.75 in order to convert between pixels and points.
            let mark_size = (settings.cell_size / 1.5) as u32;
            let mark_width = mark_glyphs.width(mark_size, &"x").unwrap_or(0.0);
            let mark_loc = [
                (settings.cell_size / 2.0) - (mark_width as f64 / 2.0),
                (settings.cell_size / 2.0) + ((mark_size as f64 * 0.75) / 2.0),
            ];
            let mark_text = Text::new_color(settings.marked_cell_background_color, mark_size);

            for col in 0..settings.cell_dimensions[0] {
                for row in 0..settings.cell_dimensions[1] {
                    let value = controller.nonogram.get([col, row]);
                    let pos = [
                        col as f64 * settings.cell_size,
                        row as f64 * settings.cell_size,
                    ];
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
                        mark_text
                            .draw(
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
                        let hint_num_width = glyphs.width(hint_num_size, &ch).unwrap_or(0.0);
                        let col_num_loc =
                            (settings.cell_size / 2.0) - (hint_num_width as f64 / 2.0);
                        let ch_x = settings.position[0] + (k as f64 * settings.cell_size) + col_num_loc;
                        let ch_y = settings.position[0] - num_pos as f64 * 20.0 - 80.0;

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
                        let ch_x = settings.position[0] - num_pos as f64 * 20.0 - 25.0;
                        let ch_y = settings.position[1] + (k as f64 * settings.cell_size) + row_num_loc;

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
            let nonogram_title_width = glyphs.width(nonogram_title_size, &nonogram_title_str).unwrap_or(0.0);
            let nonogram_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (nonogram_title_width / 2.0),
                60.0,
            ];
            Text::new_color(hex("ffffff"), nonogram_title_size)
                .draw(
                    &nonogram_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(nonogram_title_loc[0], nonogram_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw progress title.
            let progress_title_str = "PROGRESS".to_string();
            let progress_title_size = 12;
            let progress_title_width = glyphs.width(progress_title_size, &progress_title_str).unwrap_or(0.0);
            let progress_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (progress_title_width / 2.0),
                95.0,
            ];
            Text::new_color(hex("ffffff"), progress_title_size)
                .draw(
                    &progress_title_str,
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(progress_title_loc[0], progress_title_loc[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw progress.
            let progress_str = format!(
                "{} / {} ({:.2}%)",
                count_black,
                goal_black,
                (count_black as f32 / goal_black as f32) * 100.0
            );
            let progress_size = 25;
            let progress_width = glyphs.width(progress_size, &progress_str).unwrap_or(0.0);
            let progress_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (progress_width / 2.0),
                120.0,
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
            let timer_title_width = glyphs.width(timer_title_size, &timer_title_str).unwrap_or(0.0);
            let timer_title_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (timer_title_width / 2.0),
                160.0,
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
            let timer_width = glyphs.width(timer_size, &"00:00:00").unwrap_or(0.0);
            let timer_loc = [
                info_box_rect[0] + (info_box_rect[2] / 2.0) - (timer_width / 2.0),
                200.0,
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

            // Dropdown size selection menu.
            let dimensions_size = 25;
            let dimensions_pos = [settings.dimensions_dropdown_menu_box[0] + 5.0,
                settings.dimensions_dropdown_menu_box[1]
                + (settings.dimensions_dropdown_menu_box[3] / 2.0)
                + ((dimensions_size as f64 * 0.75) / 2.0)];

            match controller.dimensions_dropdown_menu {
                ButtonInteraction::None => {
                    Rectangle::new_round(hex("333333"), 5.0).draw(
                        settings.dimensions_dropdown_menu_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                    Rectangle::new_round_border(hex("333333"), 5.0, 2.0).draw(
                        settings.dimensions_dropdown_menu_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Hover => {
                    Rectangle::new_round(hex("2D2D2D"), 5.0).draw(
                        settings.dimensions_dropdown_menu_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                    Rectangle::new_round_border(hex("2D2D2D"), 5.0, 2.0).draw(
                        settings.dimensions_dropdown_menu_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Select => {
                    Rectangle::new_round(hex("333333"), 5.0).draw(
                        settings.dimensions_dropdown_menu_select_background,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                    Rectangle::new_round_border(hex("2D2D2D"), 5.0, 2.0).draw(
                        settings.dimensions_dropdown_menu_select_background,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                    Rectangle::new_round(hex("2D2D2D"), 5.0).draw(
                        settings.dimensions_dropdown_menu_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );

                    for dimension in 0..DIMENSIONS_CHOICES.len() {
                        if controller.dimensions_dropdown_options.0 == dimension {
                            match controller.dimensions_dropdown_options.1 {
                                ButtonInteraction::None => (),
                                ButtonInteraction::Hover => {
                                    Rectangle::new(hex("222222")).draw(
                                        settings.dimensions_dropdown_menu_box,
                                        &c.draw_state,
                                        c.transform.trans(0.0, dimensions_pos[1] * (dimension + 1) as f64),
                                        g,
                                    );
                                }
                                ButtonInteraction::Select => {
                                    Rectangle::new(hex("333333")).draw(
                                        settings.dimensions_dropdown_menu_box,
                                        &c.draw_state,
                                        c.transform.trans(0.0, dimensions_pos[1] * (dimension + 1) as f64),
                                        g,
                                    );
                                }
                            }
                        }
                        let dimensions_str = format!(
                            "{}x{}",
                            DIMENSIONS_CHOICES[dimension][0], DIMENSIONS_CHOICES[dimension][1]
                        );
                        Text::new_color(hex("ffffff"), dimensions_size)
                            .draw(
                                &dimensions_str,
                                glyphs,
                                &c.draw_state,
                                c.transform.trans(
                                    dimensions_pos[0],
                                    dimensions_pos[1] * (dimension + 2) as f64,
                                ),
                                g,
                            )
                            .unwrap_or_else(|_| panic!("text draw failed"));
                    }
                }
            }

            let dimensions_str = format!(
                "{}x{}",
                controller.nonogram.next_dimensions[0], controller.nonogram.next_dimensions[1]
            );
            let dimensions_size = 25;
            Text::new_color(hex("ffffff"), dimensions_size)
                .draw(
                    &dimensions_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(dimensions_pos[0], dimensions_pos[1]),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Draw dropdown arrow.
            // Reference for Material Icons: https://material.io/resources/icons/?style=baseline
            // Reference for unicode character codes: https://github.com/google/material-design-icons/blob/master/iconfont/codepoints
            let dimensions_dropdown_arrow_str = format!("\u{e5c5}");
            let dimensions_dropdown_arrow_size = 25;
            let dimensions_dropdown_arrow_width = material_icons_glyphs.width(
                dimensions_dropdown_arrow_size,
                &dimensions_dropdown_arrow_str,
            ).unwrap_or(0.0);
            Text::new_color(hex("ffffff"), dimensions_dropdown_arrow_size)
                .draw(
                    &format!("{}", dimensions_dropdown_arrow_str),
                    material_icons_glyphs,
                    &c.draw_state,
                    c.transform.trans(
                        settings.dimensions_dropdown_menu_box[0]
                            + settings.dimensions_dropdown_menu_box[2]
                            - dimensions_dropdown_arrow_width,
                        settings.dimensions_dropdown_menu_box[1]
                            + (settings.dimensions_dropdown_menu_box[3] / 2.0)
                            + (dimensions_dropdown_arrow_size as f64 * 0.75),
                    ),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));

            // Restart game button.
            match controller.restart_button {
                ButtonInteraction::None => {
                    Rectangle::new_round(hex("9e4c41"), 5.0).draw(
                        settings.restart_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Hover => {
                    Rectangle::new_round(hex("773931"), 5.0).draw(
                        settings.restart_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
                ButtonInteraction::Select => {
                    Rectangle::new_round(hex("633029"), 5.0).draw(
                        settings.restart_box,
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }

            let restart_str = "RESTART".to_string();
            let restart_size = 25;
            Text::new_color(hex("ffffff"), restart_size)
                .draw(
                    &restart_str,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(
                        settings.restart_box[0] + 5.0,
                        settings.restart_box[1]
                            + (settings.restart_box[3] / 2.0)
                            + ((restart_size as f64 * 0.75) / 2.0),
                    ),
                    g,
                )
                .unwrap_or_else(|_| panic!("text draw failed"));
        }
    }
}

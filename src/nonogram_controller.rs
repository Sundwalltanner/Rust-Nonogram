//! Responsible for dealing with all input.

use piston::input::GenericEvent;
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::common::{ButtonInteraction, Directions, DIMENSIONS_CHOICES};
use crate::nonogram_board::NonogramBoard;

/// Handles events for nonogram game.
pub struct NonogramController {
    /// Stores the nonogram state.
    pub nonogram: NonogramBoard,

    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],

    /// Stores whether a left mouse button or a right mouse button are being held down.
    mouse_d: [bool; 2],

    /// Stores whether a fill keybinding or mark keybinding are being held down.
    key_d: [bool; 2],

    /// Stores whether or not then next keyboard move will travel to the other side of the board.
    loop_back: bool,

    /// Whether or not mouse was original clicked on board.
    board_d: bool,

    /// Stores current cell type being manipulated (empty, filled, marked).
    current_action: u8,

    /// Current status of dimensions dropdown menu.
    pub dimensions_dropdown_menu: ButtonInteraction,

    /// Index of dropdown menu selected, and interaction type.
    pub dimensions_dropdown_options: (usize, ButtonInteraction),

    /// Current status of restart button.
    pub restart_button: ButtonInteraction,

    /// Current status of new game button.
    pub new_game_button: ButtonInteraction,
}

impl NonogramController {
    /// Creates a new nonogram controller.
    pub fn new(nonogram: NonogramBoard) -> NonogramController {
        NonogramController {
            nonogram,
            cursor_pos: [0.0; 2],
            mouse_d: [false; 2],
            key_d: [false; 2],
            loop_back: false,
            board_d: false,
            current_action: 0,
            dimensions_dropdown_menu: ButtonInteraction::None,
            dimensions_dropdown_options: (0, ButtonInteraction::None),
            restart_button: ButtonInteraction::None,
            new_game_button: ButtonInteraction::None,
        }
    }

    /// Handles events.
    //
    // Refer to this documentation for event traits: https://docs.rs/piston/0.49.0/piston/index.html#traits
    //
    // This triggers a Clippy warning for cognitive complexity. There's nothing that can be done about this, because
    // it's caused by the `GenericEvent` trait.
    #[allow(clippy::cognitive_complexity)]
    pub fn event<E: GenericEvent>(
        &mut self,
        board_pos: [f64; 2],
        size: [f64; 2],
        dimensions_dropdown_menu_box: [f64; 4],
        restart_box: [f64; 4],
        new_game_box: [f64; 4],
        e: &E,
    ) {
        use piston::input::{Button, Key, MouseButton};

        // Debug code for figuring out the ID of a particular event.
        //println!("{:?}", e.event_id());

        if self.nonogram.end_game_screen {
            //if true {
            if let Some(pos) = e.mouse_cursor_args() {
                self.cursor_pos = [pos[0], pos[1]];

                // Check that coordinates are inside new game button.
                if self.cursor_pos[0] >= new_game_box[0]
                    && self.cursor_pos[0] <= (new_game_box[0] + new_game_box[2])
                    && self.cursor_pos[1] >= new_game_box[1]
                    && self.cursor_pos[1] <= (new_game_box[1] + new_game_box[3])
                {
                    if self.new_game_button == ButtonInteraction::None {
                        self.new_game_button = ButtonInteraction::Hover;
                    }
                } else if self.new_game_button == ButtonInteraction::Hover
                    || (self.new_game_button == ButtonInteraction::Select && self.mouse_d[0])
                {
                    self.new_game_button = ButtonInteraction::None;
                }
            }

            // Check if left mouse button has been pressed.
            if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                self.mouse_d[0] = true;

                match self.new_game_button {
                    ButtonInteraction::Select => {
                        self.new_game_button = ButtonInteraction::None;
                    }
                    ButtonInteraction::Hover => {
                        self.new_game_button = ButtonInteraction::Select;
                    }
                    _ => (),
                }
            }

            // Check if left mouse button has been released.
            if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
                self.mouse_d[0] = false;
                self.board_d = false;

                // Check if left mouse button was released while interacting with new game button.
                if self.new_game_button == ButtonInteraction::Select {
                    self.nonogram.reset_board = true;
                    self.new_game_button = ButtonInteraction::None;
                }
            }

            if let Some(_window_closed) = e.close_args() {
                let path = Path::new("savedata.json");
                let display = path.display();

                let file = match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                    Ok(file) => file,
                };

                // Serialize it to a JSON string.
                //let j = serde_json::to_string(&self.nonogram.goal_nums);

                //println!("{:?}", j);
                let save_data = json!({
                    "dimensions": self.nonogram.dimensions,
                    "next_dimensions": self.nonogram.next_dimensions,
                    "data": self.nonogram.data,
                    "goal_nums": self.nonogram.goal_nums,
                    "count_black": self.nonogram.count_black,
                    "goal_black": self.nonogram.goal_black,
                    "duration": self.nonogram.duration,
                });

                match serde_json::to_writer_pretty(file, &save_data) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                    Ok(_) => println!("successfully wrote to {}", display),
                }

                println!("Nonogram game closed. Progress has been successfully saved.");
            }
        } else {
            // Check if mouse button has been moved within window and save its location to pos: [f64; 2]
            if let Some(pos) = e.mouse_cursor_args() {
                self.cursor_pos = [pos[0], pos[1]];

                // Find coordinates relative to upper left corner.
                let x = self.cursor_pos[0] - board_pos[0];
                let y = self.cursor_pos[1] - board_pos[1];

                // Check that coordinates are inside dimensions dropdown menu button.
                if self.cursor_pos[0] >= dimensions_dropdown_menu_box[0]
                    && self.cursor_pos[0]
                        <= (dimensions_dropdown_menu_box[0] + dimensions_dropdown_menu_box[2])
                    && self.cursor_pos[1] >= dimensions_dropdown_menu_box[1]
                    && self.cursor_pos[1]
                        <= (dimensions_dropdown_menu_box[1] + dimensions_dropdown_menu_box[3])
                {
                    if self.dimensions_dropdown_menu == ButtonInteraction::None {
                        self.dimensions_dropdown_menu = ButtonInteraction::Hover;
                    }
                } else if self.dimensions_dropdown_menu == ButtonInteraction::Hover {
                    self.dimensions_dropdown_menu = ButtonInteraction::None;
                }

                // Check that coordinates are inside sub menu of dimensions dropdown menu.
                let dropdown_sub_menu_y_min =
                    dimensions_dropdown_menu_box[1] + dimensions_dropdown_menu_box[3];
                let dropdown_sub_menu_y_max = dropdown_sub_menu_y_min
                    + (dimensions_dropdown_menu_box[3] * (DIMENSIONS_CHOICES.len() + 2) as f64);
                if self.dimensions_dropdown_menu == ButtonInteraction::Select
                    && self.cursor_pos[0] >= dimensions_dropdown_menu_box[0]
                    && self.cursor_pos[0]
                        <= (dimensions_dropdown_menu_box[0] + dimensions_dropdown_menu_box[2])
                    && self.cursor_pos[1] >= dropdown_sub_menu_y_min
                    && self.cursor_pos[1] <= dropdown_sub_menu_y_max
                {
                    let dimension_sub_index = (self.cursor_pos[1] - dropdown_sub_menu_y_min)
                        / (dimensions_dropdown_menu_box[3] + 5.0);
                    self.dimensions_dropdown_options =
                        (dimension_sub_index as usize, ButtonInteraction::Hover);
                    self.nonogram.selected_cell = None;
                } else {
                    self.dimensions_dropdown_options = (0, ButtonInteraction::None);

                    // Check that coordinates are inside board boundaries.
                    if x >= 0.0 && x < size[0] && y >= 0.0 && y < size[1] {
                        // Compute the cell position.
                        let cell_x = (x / size[0] * self.nonogram.dimensions[0] as f64) as usize;
                        let cell_y = (y / size[1] * self.nonogram.dimensions[1] as f64) as usize;
                        self.nonogram.selected_cell = Some([cell_x, cell_y]);
                        if self.nonogram.get([cell_x, cell_y]) == self.current_action
                            && self.board_d
                        {
                            if self.mouse_d[0] {
                                self.nonogram.set([cell_x, cell_y], 1);
                            } else if self.mouse_d[1] {
                                self.nonogram.set([cell_x, cell_y], 2);
                            }
                        }
                    }
                }

                // Check that coordinates are inside restart game button.
                if self.cursor_pos[0] >= restart_box[0]
                    && self.cursor_pos[0] <= (restart_box[0] + restart_box[2])
                    && self.cursor_pos[1] >= restart_box[1]
                    && self.cursor_pos[1] <= (restart_box[1] + restart_box[3])
                {
                    if self.restart_button == ButtonInteraction::None {
                        self.restart_button = ButtonInteraction::Hover;
                    }
                } else if self.restart_button == ButtonInteraction::Hover
                    || (self.restart_button == ButtonInteraction::Select && self.mouse_d[0])
                {
                    self.restart_button = ButtonInteraction::None;
                }
            }

            // Check if left mouse button has been pressed.
            if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                self.mouse_d[0] = true;

                if let Some(ind) = self.nonogram.selected_cell {
                    self.board_d = true;
                    self.current_action = self.nonogram.get(ind);
                }

                match self.dimensions_dropdown_menu {
                    ButtonInteraction::Select => {
                        self.dimensions_dropdown_menu = ButtonInteraction::None;
                        if self.dimensions_dropdown_options.1 == ButtonInteraction::Hover {
                            self.nonogram.next_dimensions =
                                DIMENSIONS_CHOICES[self.dimensions_dropdown_options.0];
                            self.dimensions_dropdown_options = (0, ButtonInteraction::None);
                        }
                    }
                    ButtonInteraction::Hover => {
                        self.dimensions_dropdown_menu = ButtonInteraction::Select;
                    }
                    _ => (),
                }

                match self.restart_button {
                    ButtonInteraction::Select => {
                        self.restart_button = ButtonInteraction::None;
                    }
                    ButtonInteraction::Hover => {
                        self.restart_button = ButtonInteraction::Select;
                    }
                    _ => (),
                }
            }

            // Check if right mouse button has been pressed.
            if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {
                self.mouse_d[1] = true;

                if let Some(ind) = self.nonogram.selected_cell {
                    self.board_d = true;
                    self.current_action = self.nonogram.get(ind);
                }
            }

            // Check if left mouse button has been released.
            if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
                self.mouse_d[0] = false;
                self.board_d = false;

                // Check if left mouse button was released while interacting with restart button.
                if self.restart_button == ButtonInteraction::Select {
                    self.nonogram.reset_board = true;
                    self.restart_button = ButtonInteraction::None;
                }
            }

            // Check if right mouse button has been released.
            if let Some(Button::Mouse(MouseButton::Right)) = e.release_args() {
                self.mouse_d[1] = false;
                self.board_d = false;
            }

            // Check if ESC key has been released.
            //
            // Refer to this documentation for keyboard key names: http://docs.piston.rs/mush/piston/input/enum.Key.html
            if let Some(Button::Keyboard(Key::Escape)) = e.release_args() {
                println!("Escape key pressed");
            }

            // Check if "Up" key has been released.
            if let Some(Button::Keyboard(Key::Up)) = e.release_args() {
                let dimensions_index = DIMENSIONS_CHOICES
                    .iter()
                    .position(|&r| r == self.nonogram.next_dimensions)
                    .unwrap();
                if dimensions_index < DIMENSIONS_CHOICES.len() - 1 {
                    self.nonogram.next_dimensions = DIMENSIONS_CHOICES[dimensions_index + 1];
                }
            }

            // Check if "Down" key has been released.
            if let Some(Button::Keyboard(Key::Down)) = e.release_args() {
                let dimensions_index = DIMENSIONS_CHOICES
                    .iter()
                    .position(|&r| r == self.nonogram.next_dimensions)
                    .unwrap();
                if dimensions_index > 0 {
                    self.nonogram.next_dimensions = DIMENSIONS_CHOICES[dimensions_index - 1];
                }
            }

            // Check if "W" key has been pressed.
            if let Some(Button::Keyboard(Key::W)) = e.press_args() {
                self.nonogram
                    .change_selected(Directions::Up, self.loop_back);
                self.loop_back = false;
                if let Some(ind) = self.nonogram.selected_cell {
                    if self.nonogram.get(ind) == self.current_action {
                        if self.key_d[0] {
                            self.nonogram.set(ind, 1);
                        } else if self.key_d[1] {
                            self.nonogram.set(ind, 2);
                        }
                    }
                }
            }

            // Check if "S" key has been pressed.
            if let Some(Button::Keyboard(Key::S)) = e.press_args() {
                self.nonogram
                    .change_selected(Directions::Down, self.loop_back);
                self.loop_back = false;
                if let Some(ind) = self.nonogram.selected_cell {
                    if self.nonogram.get(ind) == self.current_action {
                        if self.key_d[0] {
                            self.nonogram.set(ind, 1);
                        } else if self.key_d[1] {
                            self.nonogram.set(ind, 2);
                        }
                    }
                }
            }

            // Check if "A" key has been pressed.
            if let Some(Button::Keyboard(Key::A)) = e.press_args() {
                self.nonogram
                    .change_selected(Directions::Left, self.loop_back);
                self.loop_back = false;
                if let Some(ind) = self.nonogram.selected_cell {
                    if self.nonogram.get(ind) == self.current_action {
                        if self.key_d[0] {
                            self.nonogram.set(ind, 1);
                        } else if self.key_d[1] {
                            self.nonogram.set(ind, 2);
                        }
                    }
                }
            }

            // Check if "D" key has been pressed.
            if let Some(Button::Keyboard(Key::D)) = e.press_args() {
                self.nonogram
                    .change_selected(Directions::Right, self.loop_back);
                self.loop_back = false;
                if let Some(ind) = self.nonogram.selected_cell {
                    if self.nonogram.get(ind) == self.current_action {
                        if self.key_d[0] {
                            self.nonogram.set(ind, 1);
                        } else if self.key_d[1] {
                            self.nonogram.set(ind, 2);
                        }
                    }
                }
            }

            // Check if "J" key has been pressed.
            if let Some(Button::Keyboard(Key::J)) = e.press_args() {
                if !self.key_d[0] {
                    self.key_d[0] = true;
                    if let Some(ind) = self.nonogram.selected_cell {
                        self.current_action = self.nonogram.get(ind);
                        self.nonogram.set(ind, 1);
                    }
                }
            }

            // Check if "J" key has been released.
            if let Some(Button::Keyboard(Key::J)) = e.release_args() {
                self.key_d[0] = false;
            }

            // Check if "K" key has been pressed.
            if let Some(Button::Keyboard(Key::K)) = e.press_args() {
                if !self.key_d[1] {
                    self.key_d[1] = true;
                    if let Some(ind) = self.nonogram.selected_cell {
                        self.current_action = self.nonogram.get(ind);
                        self.nonogram.set(ind, 2);
                    }
                }
            }

            // Check if "K" key has been released.
            if let Some(Button::Keyboard(Key::K)) = e.release_args() {
                self.key_d[1] = false;
            }
        }

        // Check if window has been closed.
        //
        // This will check for window closure via clicking the "X" in the top right corner of the window,
        // ALT+F4, or killing the program with task manager. This won't check for closure through ESC with
        // the option ".exit_on_esc(true)" enabled in main.rs during the window's initial creation though, so
        // that option isn't enabled.
        //
        // This might be useful later if we intend to save any user progress. The program will run everything in
        // this block before it actually closes the program.
        if let Some(_window_closed) = e.close_args() {
            let path = Path::new("savedata.json");
            let display = path.display();

            let file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                Ok(file) => file,
            };

            // Serialize it to a JSON string.
            //let j = serde_json::to_string(&self.nonogram.goal_nums);

            //println!("{:?}", j);
            let save_data = json!({
                "dimensions": self.nonogram.dimensions,
                "next_dimensions": self.nonogram.next_dimensions,
                "data": self.nonogram.data,
                "goal_nums": self.nonogram.goal_nums,
                "count_black": self.nonogram.count_black,
                "goal_black": self.nonogram.goal_black,
                "duration": self.nonogram.duration,
                "end_game_screen": self.nonogram.end_game_screen,
            });

            match serde_json::to_writer_pretty(file, &save_data) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                Ok(_) => println!("successfully wrote to {}", display),
            }

            println!("Nonogram game closed. Progress has been successfully saved.");
        }

        // Check if "R" key has been released.
        if let Some(Button::Keyboard(Key::R)) = e.release_args() {
            self.nonogram.reset_board = true;
        }

        // Check if "W" key has been released.
        if let Some(Button::Keyboard(Key::W)) = e.release_args() {
            if let Some(ind) = self.nonogram.selected_cell {
                if ind[1] == 0
                    || ind[1] == self.nonogram.dimensions[1] - 1
                    || ind[0] == 0
                    || ind[0] == self.nonogram.dimensions[0] - 1
                {
                    self.loop_back = true;
                }
            }
        }

        // Check if "A" key has been released.
        if let Some(Button::Keyboard(Key::A)) = e.release_args() {
            if let Some(ind) = self.nonogram.selected_cell {
                if ind[1] == 0
                    || ind[1] == self.nonogram.dimensions[1] - 1
                    || ind[0] == 0
                    || ind[0] == self.nonogram.dimensions[0] - 1
                {
                    self.loop_back = true;
                }
            }
        }

        // Check if "S" key has been released.
        if let Some(Button::Keyboard(Key::S)) = e.release_args() {
            if let Some(ind) = self.nonogram.selected_cell {
                if ind[1] == 0
                    || ind[1] == self.nonogram.dimensions[1] - 1
                    || ind[0] == 0
                    || ind[0] == self.nonogram.dimensions[0] - 1
                {
                    self.loop_back = true;
                }
            }
        }

        // Check if "D" key has been released.
        if let Some(Button::Keyboard(Key::D)) = e.release_args() {
            if let Some(ind) = self.nonogram.selected_cell {
                if ind[1] == 0
                    || ind[1] == self.nonogram.dimensions[1] - 1
                    || ind[0] == 0
                    || ind[0] == self.nonogram.dimensions[0] - 1
                {
                    self.loop_back = true;
                }
            }
        }

        // Check if "J" key has been released.
        if let Some(Button::Keyboard(Key::J)) = e.release_args() {
            self.key_d[0] = false;
        }

        // Check if "K" key has been released.
        if let Some(Button::Keyboard(Key::K)) = e.release_args() {
            self.key_d[1] = false;
        }
    }
}

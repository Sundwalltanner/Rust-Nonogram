//! Responsible for dealing with all input.

use piston::input::{Button, GenericEvent, Key, MouseButton};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::common::{ButtonInteraction, Cell, Directions, DIMENSIONS_CHOICES};
use crate::nonogram_board::NonogramBoard;

/// Handles nonogram keybindings.
pub struct NonogramControls {
    /// Keybinding for filling the cell that the mouse cursor is hovering over.
    pub mouse_fill: Button,

    /// Keybinding for marking the cell that the mouse cursor is hovering over.
    pub mouse_mark: Button,

    /// Keybinding for moving the selected cell focus up.
    pub move_up: Button,

    /// Keybinding for moving the selected cell focus down.
    pub move_down: Button,

    /// Keybinding for moving the selected cell focus left.
    pub move_left: Button,

    /// Keybinding for moving the selected cell focus right.
    pub move_right: Button,

    /// Keybinding for filling the currently selected cell.
    pub key_fill: Button,

    /// Keybinding for marking the currently selected cell.
    pub key_mark: Button,

    /// Keybinding for generating a new board and starting from scratch.
    pub restart: Button,

    /// Keybinding for increasing the dimensions of the next board to be generated.
    pub dim_up: Button,

    /// Keybinding for decreasing the dimensions of the next board to be generated.
    pub dim_down: Button,
}

/// Default implementation for NonogramControls.
impl Default for NonogramControls {
    /// Creates new control handler.
    fn default() -> Self {
        NonogramControls {
            mouse_fill: Button::Mouse(MouseButton::Left),
            mouse_mark: Button::Mouse(MouseButton::Right),
            move_up: Button::Keyboard(Key::W),
            move_down: Button::Keyboard(Key::S),
            move_left: Button::Keyboard(Key::A),
            move_right: Button::Keyboard(Key::D),
            key_fill: Button::Keyboard(Key::J),
            key_mark: Button::Keyboard(Key::K),
            restart: Button::Keyboard(Key::R),
            dim_up: Button::Keyboard(Key::Up),
            dim_down: Button::Keyboard(Key::Down),
        }
    }
}

/// Handles events for nonogram game.
pub struct NonogramController {
    /// Stores the keybindings.
    pub controls: NonogramControls,

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
    current_action: Cell,

    /// Current status of dimensions dropdown menu.
    pub dimensions_dropdown_menu: ButtonInteraction,

    /// Index of dropdown menu selected, and interaction type.
    pub dimensions_dropdown_options: (usize, ButtonInteraction),

    /// Current status of restart button.
    pub restart_button: ButtonInteraction,

    /// Current status of new game button.
    pub new_game_button: ButtonInteraction,
}

/// Implementation for NonogramController.
impl NonogramController {
    /// Creates a new nonogram controller.
    pub fn new(nonogram: NonogramBoard) -> NonogramController {
        NonogramController {
            controls: Default::default(),
            nonogram,
            cursor_pos: [0.0; 2],
            mouse_d: [false; 2],
            key_d: [false; 2],
            loop_back: false,
            board_d: false,
            current_action: Cell::Empty,
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
                                self.nonogram.set([cell_x, cell_y], Cell::Filled);
                            } else if self.mouse_d[1] {
                                self.nonogram.set([cell_x, cell_y], Cell::Marked);
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
            if Some(self.controls.mouse_fill) == e.press_args() {
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
            if Some(self.controls.mouse_mark) == e.press_args() {
                self.mouse_d[1] = true;

                if let Some(ind) = self.nonogram.selected_cell {
                    self.board_d = true;
                    self.current_action = self.nonogram.get(ind);
                }
            }

            // Check if left mouse button has been released.
            if Some(self.controls.mouse_fill) == e.release_args() {
                self.mouse_d[0] = false;
                self.board_d = false;

                // Check if left mouse button was released while interacting with restart button.
                if self.restart_button == ButtonInteraction::Select {
                    self.nonogram.reset_board = true;
                    self.restart_button = ButtonInteraction::None;
                }
            }

            // Check if right mouse button has been released.
            if Some(self.controls.mouse_mark) == e.release_args() {
                self.mouse_d[1] = false;
                self.board_d = false;
            }

            // Check if ESC key has been released.
            //
            // Refer to this documentation for keyboard key names: http://docs.piston.rs/mush/piston/input/enum.Key.html
            if let Some(Button::Keyboard(Key::Escape)) = e.release_args() {
                println!("Escape key pressed");
            }

            // Check if key for increasing dimensions has been released.
            if Some(self.controls.dim_up) == e.release_args() {
                let dimensions_index = DIMENSIONS_CHOICES
                    .iter()
                    .position(|&r| r == self.nonogram.next_dimensions)
                    .unwrap();
                if dimensions_index < DIMENSIONS_CHOICES.len() - 1 {
                    self.nonogram.next_dimensions = DIMENSIONS_CHOICES[dimensions_index + 1];
                }
            }

            // Check if key for decreasing dimensions has been released.
            if Some(self.controls.dim_down) == e.release_args() {
                let dimensions_index = DIMENSIONS_CHOICES
                    .iter()
                    .position(|&r| r == self.nonogram.next_dimensions)
                    .unwrap();
                if dimensions_index > 0 {
                    self.nonogram.next_dimensions = DIMENSIONS_CHOICES[dimensions_index - 1];
                }
            }

            // Check if move up key has been pressed.
            if Some(self.controls.move_up) == e.press_args() {
                self.nonogram
                    .change_selected(Directions::Up, self.loop_back);
            }

            // Check if move down key has been pressed.
            if Some(self.controls.move_down) == e.press_args() {
                self.nonogram
                    .change_selected(Directions::Down, self.loop_back);
            }

            // Check if move left key has been pressed.
            if Some(self.controls.move_left) == e.press_args() {
                self.nonogram
                    .change_selected(Directions::Left, self.loop_back);
            }

            // Check if move right key has been pressed
            if Some(self.controls.move_right) == e.press_args() {
                self.nonogram
                    .change_selected(Directions::Right, self.loop_back);
            }

            // Detect keyboard movement keys
            if e.press_args() == Some(self.controls.move_up)
                || e.press_args() == Some(self.controls.move_down)
                || e.press_args() == Some(self.controls.move_left)
                || e.press_args() == Some(self.controls.move_right)
            {
                self.loop_back = false;
                if let Some(ind) = self.nonogram.selected_cell {
                    if self.nonogram.get(ind) == self.current_action {
                        if self.key_d[0] {
                            self.nonogram.set(ind, Cell::Filled);
                        } else if self.key_d[1] {
                            self.nonogram.set(ind, Cell::Marked);
                        }
                    }
                }
            }

            // Check if fill key has been pressed.
            if Some(self.controls.key_fill) == e.press_args() && !self.key_d[0] {
                self.key_d[0] = true;
                if let Some(ind) = self.nonogram.selected_cell {
                    self.current_action = self.nonogram.get(ind);
                    self.nonogram.set(ind, Cell::Filled);
                }
            }

            // Check if mark key has been pressed.
            if Some(self.controls.key_mark) == e.press_args() && !self.key_d[1] {
                self.key_d[1] = true;
                if let Some(ind) = self.nonogram.selected_cell {
                    self.current_action = self.nonogram.get(ind);
                    self.nonogram.set(ind, Cell::Marked);
                }
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
                Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
                Ok(_) => println!("Successfully wrote to {}", display),
            }

            println!("Nonogram game closed.");
        }

        // Check if restart key has been released.
        if Some(self.controls.restart) == e.release_args() {
            self.nonogram.reset_board = true;
        }

        if e.release_args() == Some(self.controls.move_up)
            || e.release_args() == Some(self.controls.move_down)
            || e.release_args() == Some(self.controls.move_left)
            || e.release_args() == Some(self.controls.move_right)
        {
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

        // Check if fill key has been released.
        if Some(self.controls.key_fill) == e.release_args() {
            self.key_d[0] = false;
        }

        // Check if mark key has been released.
        if Some(self.controls.key_mark) == e.release_args() {
            self.key_d[1] = false;
        }
    }
}

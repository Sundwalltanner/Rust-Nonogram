use piston::input::GenericEvent;

use crate::nonogram_board::NonogramBoard;

/// Handles events for nonogram game.
pub struct NonogramController {
    /// Stores the nonogram state.
    pub nonogram: NonogramBoard,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
    /// Stores whether a left mouse button or a right mouse button are being held down.
    mouse_d: [bool; 2],
    /// Stores current cell type being manipulated (empty, filled, marked).
    current_action: u8,
}

impl NonogramController {
    /// Creates a new nonogram controller.
    pub fn new(nonogram: NonogramBoard) -> NonogramController {
        NonogramController {
            nonogram,
            selected_cell: None,
            cursor_pos: [0.0; 2],
            mouse_d: [false; 2],
            current_action: 0,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, board_pos: [f64; 2], size: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = [pos[0], pos[1]];

            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - board_pos[0];
            let y = self.cursor_pos[1] - board_pos[1];

            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size[0] && y >= 0.0 && y < size[1] {
                // Compute the cell position.
                let cell_x = (x / size[0] * self.nonogram.dimensions[0] as f64) as usize;
                let cell_y = (y / size[1] * self.nonogram.dimensions[1] as f64) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
                if self.nonogram.get([cell_x, cell_y]) == self.current_action {
                    if self.mouse_d[0] == true {
                        self.nonogram.set([cell_x, cell_y], 1);
                    } else if self.mouse_d[1] == true {
                        self.nonogram.set([cell_x, cell_y], 2);
                    }
                }
            } else {
                self.selected_cell = None;
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                self.mouse_d[0] = true;
                self.current_action = self.nonogram.get(ind);
            }
        }
        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                self.mouse_d[1] = true;
                self.current_action = self.nonogram.get(ind);
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
                self.mouse_d[0] = false;
        }
        if let Some(Button::Mouse(MouseButton::Right)) = e.release_args() {
                self.mouse_d[1] = false;
        }
    }
}

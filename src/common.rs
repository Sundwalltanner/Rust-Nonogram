#[derive(PartialEq)]
pub enum ButtonInteraction {
    None,
    Hover,
    Select,
}

pub const INITIAL_BOARD_DIMENSIONS: [usize; 2] = [15, 10];
pub const INITIAL_WINDOW_SIZE: [u32; 2] = [1200, 875];
pub const BOARD_SIZE: f64 = 1200.0;
pub const DIMENSIONS_CHOICES: [[usize; 2]; 11] = [
    [5, 5],
    [10, 5],
    [10, 10],
    [15, 10],
    [15, 15],
    [20, 15],
    [20, 20],
    [25, 20],
    [25, 25],
    [30, 25],
    [30, 30],
];
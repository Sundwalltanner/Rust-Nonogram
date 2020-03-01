//! Contains enums and constants frequently used throughout the program.
//!
//! The following files use these parts:
//! - [main], utilizes [INITIAL_WINDOW_SIZE] and [INITIAL_BOARD_DIMENSIONS] for window and board initialization.
//! - [nonogram_board_view], utilizes [BOARD_SIZE], [DIMENSIONS_CHOICES], [IMAGE_PRE], [IMAGE_NAMES], and [ButtonInteraction].
//!
//! [main]: ../fn.main.html
//! [nonogram_board_view]: ../nonogram_board_view/index.html
//! [ButtonInteraction]: enum.ButtonInteraction.html
//! [BOARD_SIZE]: constant.BOARD_SIZE.html
//! [DIMENSIONS_CHOICES]: constant.DIMENSIONS_CHOICES.html
//! [IMAGE_NAMES]: constant.IMAGE_NAMES.html
//! [IMAGE_PRE]: constant.IMAGE_PRE.html
//! [INITIAL_BOARD_DIMENSIONS]: constant.INITIAL_BOARD_DIMENSIONS.html
//! [INITIAL_WINDOW_SIZE]: constant.INITIAL_WINDOW_SIZE.html

/// Determines the current status of an interactable button.
#[derive(PartialEq)]
pub enum ButtonInteraction {
    /// Button is not being interacted with.
    None,

    /// Button has cursor hovering over it.
    Hover,

    /// Select, determined on a per button basis. Usually means the button has been clicked.
    Select,
}

/// Determines the columns and rows in a fresh run of the program.
/// If any save data exists, it will overwrite these values with the most recently used dimensions.
pub const INITIAL_BOARD_DIMENSIONS: [usize; 2] = [15, 10];

/// Determines the initial window size. Unlike the value for the initial board dimensions,
/// this is not saved. This will be the initial size of the window every time the program is run.
pub const INITIAL_WINDOW_SIZE: [u32; 2] = [1200, 875];

/// This determines both the width and height of the board displayed within the window while playing
/// the game. The overall board size is calculated by taking both this and the board dimensions into
/// account.
pub const BOARD_SIZE: f64 = 1200.0;

/// The options that will show up when the user clicks on the dropdown menu for selecting the board
/// dimensions. This can be manipulated, and the game will dynamically take the adjustment into account.
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

/// Part 1 of random string generation for the win screen. Nonogram puzzles usually result in an image of
/// something being produced. Due to the random generation I use to produce my puzzles, it's a miracle if
/// anything's actually produced in the end. This is just a joke.
pub const IMAGE_PRE: [[&str; 2]; 9] = [
    ["I think it's", "?"],
    ["It looks just like", "!"],
    ["It might be", "?"],
    ["It's obviously", "!"],
    ["It's supposed to be", "."],
    ["Is that...", "?"],
    ["Wow! I've never seen", " with this much depth!"],
    ["This drawing of", " sure is impressive!"],
    ["How much are you selling this drawing of", " for?"],
];

/// Part 2 of random string generation for the win screen.
pub const IMAGE_NAMES: [&str; 29] = [
    "an apple",
    "a flag",
    "a pair of boots",
    "an autumn leaf",
    "a megaphone",
    "a compass",
    "a melon",
    "a computer mouse",
    "a keyboard",
    "a chair",
    "a spatula",
    "a house",
    "a trash can",
    "a frog",
    "a measuring cup",
    "a refrigerator",
    "a table",
    "Abraham Lincoln",
    "a border collie",
    "a pomeranian",
    "a golden retriever",
    "a german shepherd",
    "a chihuahua",
    "a corgi",
    "a rose",
    "a dandelion",
    "a rubber ducky",
    "a parakeet",
    "an octopus",
];

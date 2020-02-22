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
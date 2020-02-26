//! Responsible for everything that isn't input or graphics.

use std::time::{Duration, Instant};
use rand::distributions::{Bernoulli, Distribution};
use std::fs;
use serde::{Deserialize, Serialize};

/// Contains the information we're going to save in between each session.
#[derive(Serialize, Deserialize)]
pub struct SavedBoard {
    pub dimensions: [usize; 2],
    pub next_dimensions: [usize; 2],
    pub data: Vec<Vec<u8>>,
    pub goal_nums: Vec<Vec<Vec<i8>>>,
    pub count_black: u64,
    pub goal_black: u64,
    pub duration: Duration,
    pub end_game_screen: bool,
}

/// Contains all logic pertaining to the nonogram board.
pub struct NonogramBoard {
    /// Current nonogram board columns and rows.
    pub dimensions: [usize; 2],

    /// Next nonogram board columns and rows.
    pub next_dimensions: [usize; 2],

    /// Contains all cell data of the current nonogram board.
    /// 
    /// A cell can be empty(0), filled(1), or marked(2).
    pub data: Vec<Vec<u8>>,

    /// The maximum hint numbers for the columns and rows depending on the board dimensions.
    /// 
    /// This is equal to the number of cells in that column or row divided by 2 and rounded up.
    /// 
    /// Example: The maximum hint numbers in a column of 5 cells is 3.
    /// 
    /// This is calculated on initialization like this:
    /// ```
    /// self.nums_per[0] = (self.dimensions[1] as f64 / 2.0_f64).round() as u64;
    /// self.nums_per[1] = (self.dimensions[0] as f64 / 2.0_f64).round() as u64;
    /// ```
    pub nums_per: [u64; 2],

    /// The goal hint numbers. If a value is negative, it's crossed out.
    /// ```
    /// goal_nums[column_or_row][which_column_or_row][which_number];
    /// goal_nums[0][1][2] => Third hint number in second column
    /// goal_nums[1][2][3] => Fourth hint number in third row
    /// ```
    pub goal_nums: Vec<Vec<Vec<i8>>>,

    /// The current hint numbers.
    /// 
    /// Because we don't generate unique goal states, this is what we compare with the goal_nums in order
    /// to determine whether or not we've reached a valid goal state.
    pub current_nums: Vec<Vec<Vec<i8>>>,

    /// Time that the current game started at.
    pub game_start: Option<Instant>,

    /// Unused at the moment.
    pub last_time: Option<Instant>,

    /// Time that the most recent game ended at.
    pub game_end: Option<Instant>,

    /// True if we're at the end game screen.
    /// False if we're not at the end game screen.
    pub end_game_screen: bool,

    /// The amount of time (in seconds) that's passed since the current game has started.
    pub duration: Duration,

    /// Whether or not we want to restart the board. This is checked in the main waiting loop.
    pub reset_board: bool,

    /// How many cells the player has filled in.
    pub count_black: u64,

    /// How many filled in cells the goal state has.
    pub goal_black: u64,

    /// The ratio between the number of filled in cells in the goal state and the total number of cells on the board.
    pub init_ratio: f64,
}

/// NonogramBoard functionality.
impl NonogramBoard {
    pub fn new(next_dimensions: [usize; 2], reset_board: bool) -> NonogramBoard {
        let mut board = NonogramBoard {
            dimensions: next_dimensions,
            next_dimensions: next_dimensions,
            data: vec![vec![]],
            nums_per: [0; 2],
            goal_nums: vec![vec![vec![]]],
            current_nums: vec![vec![vec![]]],
            game_start: None,
            game_end: None,
            end_game_screen: false,
            duration: Duration::from_secs(0),
            reset_board: reset_board,
            last_time: None,
            count_black: 0,
            goal_black: 0,
            init_ratio: 0.5,
        };
        board.init_new();
        board
    }

    /// Initialize values that cannot be initialized in the constructor.
    /// This function is called within the constructor.
    fn init_new(&mut self) {
        self.data.clear();
        self.goal_nums.clear();
        self.current_nums.clear();

        let save_data = fs::read_to_string("savedata.json").unwrap_or("".to_string());

        // If there is no save data file or if we're generating a brand-new board.
        if save_data.is_empty() || self.reset_board {
            for _col in 0..self.dimensions[0] {
                self.data.push(vec![0; self.dimensions[1]]);
            }
    
            self.nums_per[0] = (self.dimensions[1] as f64 / 2.0_f64).round() as u64;
            self.nums_per[1] = (self.dimensions[0] as f64 / 2.0_f64).round() as u64;
    
            for i in 0..2 {
                self.goal_nums
                    .push(vec![vec![0; self.nums_per[i] as usize]; self.dimensions[i]]);
                self.current_nums
                    .push(vec![vec![0; self.nums_per[i] as usize]; self.dimensions[i]]);
            }
            
            self.initialize();
        } else {
            let v: SavedBoard = serde_json::from_str(&save_data).expect("Your savedata.json file is incompatible. Delete it.");
            self.dimensions = v.dimensions;
            self.next_dimensions = v.next_dimensions;
            self.data = v.data;
            self.goal_nums = v.goal_nums;
            self.count_black = v.count_black;
            self.goal_black = v.goal_black;
            self.duration = v.duration;
            self.nums_per[0] = (self.dimensions[1] as f64 / 2.0_f64).round() as u64;
            self.nums_per[1] = (self.dimensions[0] as f64 / 2.0_f64).round() as u64;
            self.game_start = Some(Instant::now() - self.duration);
            self.end_game_screen = v.end_game_screen;
        }
    }

    /// Compare the goal state to the current state of the hint numbers. Return true if the player
    /// has won. Return false if the player hasn't won.
    pub fn check_win(&self) -> bool {
        for i in 0..2 {
            for k in 0..self.dimensions[i] {
                if !self.goal_nums[i][k].iter().zip(self.current_nums[i][k].iter()).all(|(a,b)| a.abs() == b.abs()) {
                    return false;
                }
            }
        }

        true
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if self.data[ind[0]][ind[1]] != 0 {
            if self.data[ind[0]][ind[1]] == 1 && self.count_black != 0 {
                self.count_black -= 1;
            }
            self.data[ind[0]][ind[1]] = 0;
        } else {
            if val == 1 {
                self.count_black += 1;
            }
            self.data[ind[0]][ind[1]] = val;
        }
        self.current_nums = self.get_nums();
        self.update_crossouts();

        self.end_game_screen = self.check_win();
        if self.end_game_screen {
            self.game_end = Some(Instant::now());
        }
    }

    /// Get cell value.
    pub fn get(&self, ind: [usize; 2]) -> u8 {
        self.data[ind[0]][ind[1]]
    }

    /// Setup randomly generated goal nonogram.
    pub fn set_goal(&mut self) {
        let rng = Bernoulli::new(self.init_ratio).unwrap();
        for col in 0..self.dimensions[0] {
            for row in 0..self.dimensions[1] {
                if rng.sample(&mut rand::thread_rng()) {
                    self.data[col][row] = 1;
                    self.goal_black += 1;
                }
            }
        }
    }

    /// Clear board and set all cells to default state.
    pub fn wipe_board(&mut self) {
        for col in 0..self.dimensions[0] {
            for row in 0..self.dimensions[1] {
                self.data[col][row] = 0;
            }
        }
    }

    /// Find the current black box groupings in order to find correct values
    /// for numbers nearby columns and rows.
    pub fn get_nums(&self) -> Vec<Vec<Vec<i8>>> {
        let mut nums = vec![vec![vec![0; self.nums_per[0] as usize]; self.dimensions[0]]];

        nums.push(vec![vec![0; self.nums_per[1] as usize]; self.dimensions[1]]);

        // Get column nums.
        for col in 0..self.dimensions[0] {
            let mut num_hint = 0;
            let mut filling = false;
            for row in 0..self.dimensions[1] {
                if self.data[col][row] == 1 {
                    if filling == false {
                        filling = true;
                    }
                    nums[0][col][num_hint] += 1;
                } else {
                    if filling {
                        filling = false;
                        num_hint += 1;
                    }
                }
            }
        }

        // Get row nums.
        for row in 0..self.dimensions[1] {
            let mut num_hint = 0;
            let mut filling = false;
            for col in 0..self.dimensions[0] {
                if self.data[col][row] == 1 {
                    if filling == false {
                        filling = true;
                    }
                    nums[1][row][num_hint] += 1;
                } else {
                    if filling {
                        filling = false;
                        num_hint += 1;
                    }
                }
            }
        }
        nums
    }

    /// Hint numbers are stored as signed integers, which means that we utilize the negative values in order to
    /// declare some of the hint numbers as crossed out in order to assist the player.
    /// 
    /// For example, if the hint numbers in a row are [1, 3, 1, 2] and the player fills in a single square, in
    /// the background, that first 1 will actually be -1. It will be displayed to the user as 1 still, but it will
    /// have a different color to it in order to inform the user that it's crossed out.
    pub fn update_crossouts(&mut self) {
        // Check column nums.
        for k in 0..self.dimensions[0] {
            let mut match_count = false;
            let mut current_it = 0usize;
            let mut goal_it = current_it;
            let mut match_it = current_it;

            // If we have more sequences than are in the goal, then they're all wrong.
            if self.current_nums[0][k].iter().filter(|&n| *n != 0).count()
                <= self.goal_nums[0][k].iter().filter(|&n| *n != 0).count()
            {
                match_count = true;
            }
            while goal_it < self.nums_per[0] as usize {
                if self.goal_nums[0][k][goal_it] != 0 {
                    while current_it < self.nums_per[0] as usize {
                        if self.goal_nums[0][k][goal_it].abs()
                            == self.current_nums[0][k][current_it]
                            && match_count
                        {
                            if self.goal_nums[0][k][goal_it] > 0 {
                                self.goal_nums[0][k][goal_it] *= -1;
                            }
                            match_it = current_it + 1;
                            break;
                        } else {
                            self.goal_nums[0][k][goal_it] =
                                self.goal_nums[0][k][goal_it].abs();
                        }
                        current_it += 1;
                    }
                }
                current_it = match_it;
                goal_it += 1;
            }
        }

        // Check row nums.
        for k in 0..self.dimensions[1] {
            let mut match_count = false;
            let mut current_it = 0usize;
            let mut goal_it = current_it;
            let mut match_it = current_it;

            // If we have more sequences than are in the goal, then they're all wrong.
            if self.current_nums[1][k].iter().filter(|&n| *n != 0).count()
                <= self.goal_nums[1][k].iter().filter(|&n| *n != 0).count()
            {
                match_count = true;
            }
            while goal_it < self.nums_per[1] as usize {
                if self.goal_nums[1][k][goal_it] != 0 {
                    while current_it < self.nums_per[1] as usize{
                        if self.goal_nums[1][k][goal_it].abs()
                            == self.current_nums[1][k][current_it]
                            && match_count
                        {
                            if self.goal_nums[1][k][goal_it] > 0 {
                                self.goal_nums[1][k][goal_it] *= -1;
                            }
                            match_it = current_it + 1;
                            break;
                        } else {
                            self.goal_nums[1][k][goal_it] =
                                self.goal_nums[1][k][goal_it].abs();
                        }
                        current_it += 1;
                    }
                }
                current_it = match_it;
                goal_it += 1;
            }
        }
    }

    /// Initialize nonogram board.
    pub fn initialize(&mut self) {
        self.set_goal();
        self.goal_nums = self.get_nums();
        self.wipe_board();
        self.game_start = Some(Instant::now());
        self.reset_board = false;
    }
}

use chrono::{DateTime, Duration, Utc};
use rand::distributions::{Bernoulli, Distribution};
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct NonogramBoard {
    pub data: [[u8; 10]; 10],
    pub goal_nums: [[[u8; 5]; 10]; 2],
    pub current_nums: [[[u8; 5]; 10]; 2],
    pub game_start: Option<DateTime<Utc>>,
    pub last_time: Option<DateTime<Utc>>,
    pub game_end: Option<DateTime<Utc>>,
    pub count_black: u8,
    pub goal_black: u8,
}
impl NonogramBoard {
    pub fn new() -> Self {
        Self { 
            data: [[0; 10]; 10],
            goal_nums: [[[1; 5]; 10]; 2],
            current_nums: [[[0; 5]; 10]; 2],
            game_start: Some(Utc::now()),
            game_end: None,
            last_time: None,
            count_black: 0,
            goal_black: 0,
        }
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if self.data[ind[1]][ind[0]] != 0 {
            if self.data[ind[1]][ind[0]] == 1 && self.count_black != 0 {
                self.count_black -= 1;
            }
            self.data[ind[1]][ind[0]] = 0;
        } else {
            if val == 1 {
                self.count_black += 1;
            }
            self.data[ind[1]][ind[0]] = val;
        }

        self.current_nums = self.get_nums();
    }

    /// Get cell value.
    pub fn get(&self, ind: [usize; 2]) -> u8 {
        self.data[ind[1]][ind[0]]
    }

    /// Setup randomly generated goal nonogram.
    pub fn set_goal(&mut self) {
        let mut rng = Bernoulli::new(0.5).unwrap();
        for row in 0..10 {
            for col in 0..10 {
                if rng.sample(&mut rand::thread_rng()) {
                    self.data[row][col] = 1;
                    self.goal_black += 1;
                }
            }
        }
    }

    /// Find the current black box groupings in order to find correct values
    /// for numbers nearby columns and rows.
    pub fn get_nums(&self) -> [[[u8; 5]; 10]; 2] {
        let mut nums = [[[0; 5]; 10]; 2];
        let mut filling = false;

        for row in 0..10 {
            let mut num_hint = 4;
            for col in 0..10 {
                if self.data[row][col] == 1 {
                    if filling == false {
                        filling = true;
                    }
                    nums[0][row][num_hint] += 1;
                } else {
                    if filling {
                        filling = false;
                        num_hint -= 1;
                    }
                }
            }
        }

        for col in 0..10 {
            let mut num_hint = 4;
            for row in 0..10 {
                if self.data[row][col] == 1 {
                    if filling == false {
                        filling = true;
                    }
                    nums[1][col][num_hint] += 1;
                } else {
                    if filling {
                        filling = false;
                        num_hint -= 1;
                    }
                }
            }
        }
        nums
    }

    /// Initialize nonogram board.
    pub fn initialize(&mut self) -> Self {
        self.set_goal();
        self.goal_nums = self.get_nums();
        Self { 
            data: self.data,
            goal_nums: self.goal_nums,
            current_nums: self.current_nums,
            game_start: self.game_start,
            game_end: self.game_end,
            last_time: self.last_time,
            count_black: self.count_black,
            goal_black: self.count_black,
        }
    }
}

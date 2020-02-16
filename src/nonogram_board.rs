use chrono::{DateTime, Duration, Utc};
use rand::distributions::{Bernoulli, Distribution};

//#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonogramBoard {
    pub dimensions: [usize; 2],
    pub data: Vec<Vec<u8>>,
    pub nums_per: [u64; 2],
    pub goal_nums: Vec<Vec<Vec<u64>>>,
    pub current_nums: Vec<Vec<Vec<u64>>>,
    pub game_start: Option<DateTime<Utc>>,
    pub last_time: Option<DateTime<Utc>>,
    pub game_end: Option<DateTime<Utc>>,
    pub count_black: u64,
    pub goal_black: u64,
    pub init_ratio: f64,
}

impl NonogramBoard {
    pub fn new() -> NonogramBoard {
        let mut board = NonogramBoard {
            dimensions: [15, 10],
            data: vec![vec![]],
            nums_per: [0; 2],
            goal_nums: vec![vec![vec![]]],
            current_nums: vec![vec![vec![]]],
            game_start: None,
            game_end: None,
            last_time: None,
            count_black: 0,
            goal_black: 0,
            init_ratio: 0.5,
        };
        board.init_vecs();
        board
    }

    fn init_vecs(&mut self) {
        self.data.clear();
        self.goal_nums.clear();
        self.current_nums.clear();

        for col in 0..self.dimensions[0] {
            self.data.push(vec![0; self.dimensions[1]]);
        }

        self.nums_per[0] = (self.dimensions[1] as f64 / 2.0_f64).round() as u64;
        self.nums_per[1] = (self.dimensions[0] as f64 / 2.0_f64).round() as u64;

        for i in 0..2 {
            self.goal_nums.push(vec![vec![0; self.nums_per[i] as usize]; self.dimensions[i]]);
            self.current_nums.push(vec![vec![0; self.nums_per[i] as usize]; self.dimensions[i]]);
        }
    }

    pub fn check_win(&self) -> bool {
        // Check column nums.
        for k in 0..self.dimensions[0] {
            for i in 0..self.nums_per[0] as usize {
                if self.goal_nums[0][k][i] != self.current_nums[0][k][i] {
                    return false;
                }
            }
        }

        // Check row nums.
        for k in 0..self.dimensions[1] {
            for i in 0..self.nums_per[1] as usize {
                if self.goal_nums[1][k][i] != self.current_nums[1][k][i] {
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

        if self.check_win() {
            self.game_end = Some(Utc::now());
        }
    }

    /// Get cell value.
    pub fn get(&self, ind: [usize; 2]) -> u8 {
        self.data[ind[0]][ind[1]]
    }

    /// Setup randomly generated goal nonogram.
    pub fn set_goal(&mut self) {
        let mut rng = Bernoulli::new(self.init_ratio).unwrap();
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
    pub fn get_nums(&self) -> Vec<Vec<Vec<u64>>> {
        let mut nums = vec![vec![vec![0; self.nums_per[0] as usize]; self.dimensions[0]]];

        nums.push(vec![vec![0; self.nums_per[1] as usize]; self.dimensions[1]]);

        // Get column nums.
        for col in 0..self.dimensions[0] {
            let mut num_hint = (self.nums_per[0] - 1) as usize;
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
                        if num_hint != 0 {
                            num_hint -= 1;
                        }
                    }
                }
            }
        }

        // Get row nums.
        for row in 0..self.dimensions[1] {
            let mut num_hint = (self.nums_per[1] - 1) as usize;
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
                        if num_hint != 0 {
                            num_hint -= 1;
                        }
                    }
                }
            }
        }
        nums
    }

    /// Initialize nonogram board.
    pub fn initialize(&mut self) {
        self.set_goal();
        self.goal_nums = self.get_nums();
        //self.wipe_board();
        self.game_start = Some(Utc::now());
    }
}

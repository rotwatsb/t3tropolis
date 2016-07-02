use rand::{OsRng, Rng};

pub const ROWS: usize = 22;
pub const COLS: usize = 10;

#[derive(Copy, Clone, PartialEq, RustcDecodable, RustcEncodable, Debug)]
pub enum Cell {
    E, I, J, L, O, S, T, Z,
}

pub type Shape = [[[u8; 4]; 4]; 4];

pub const ISHAPE: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 1],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 1],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

pub const JSHAPE: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 1, 0],
     [0, 0, 0, 0]]
];

pub const LSHAPE: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [1, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

pub const OSHAPE: Shape = [
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

pub const SSHAPE: Shape = [
    [[0, 1, 1, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 1, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];

pub const TSHAPE: Shape = [
    [[0, 1, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],

    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];

pub const ZSHAPE: Shape = [
    [[1, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];

fn cell_of_shape(s: Shape) -> Cell {
    match s {
        ISHAPE => Cell::I,
        JSHAPE => Cell::J,
        LSHAPE => Cell::L,
        OSHAPE => Cell::O,
        SSHAPE => Cell::S,
        TSHAPE => Cell::T,
        ZSHAPE => Cell::Z,
        _ => Cell::I,
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub enum TradeState {
    NoTrade,
    Pending(usize),
    Confirm(usize),
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct PlayerState {
    pub board: [[Cell; COLS]; ROWS],
    pub tetromino: (Shape, usize),
    pub next_tetromino: (Shape, usize, TradeState),
    pub tetro_pos: (i8, i8),
    pub id: usize,
}

impl PlayerState {
    pub fn new(id: usize) -> PlayerState {
        PlayerState {
            board: [[Cell::E; COLS]; ROWS],
            tetromino: (ISHAPE, 0),
            next_tetromino: (ISHAPE, 0, TradeState::NoTrade),
            tetro_pos: (ROWS as i8 - 3, COLS as i8 / 2 - 1),
            id: id,
        }
    }

    pub fn begin(&mut self) {
        self.select_next_shape();
        self.new_tetromino();
    }

    fn select_next_shape(&mut self) {
        let mut rng = OsRng::new().unwrap();
        self.next_tetromino =
            match (rng.next_f32() * 7.0) as u8 {
                0 => (ISHAPE, 0, TradeState::NoTrade),
                1 => (JSHAPE, 0, TradeState::NoTrade),
                2 => (LSHAPE, 0, TradeState::NoTrade),
                3 => (OSHAPE, 0, TradeState::NoTrade),
                4 => (SSHAPE, 0, TradeState::NoTrade),
                5 => (TSHAPE, 0, TradeState::NoTrade),
                6 => (ZSHAPE, 0, TradeState::NoTrade),
                _ => (ISHAPE, 0, TradeState::NoTrade),
            };
    }

    fn new_tetromino(&mut self) {
        self.tetromino.0 = self.next_tetromino.0;
        self.tetromino.1 = self.next_tetromino.1;
        self.select_next_shape();
    }

    pub fn toggle_swap(&mut self, d: isize, n: isize) {
        let target: usize = ((self.id as isize + d + n) % n) as usize;
        match self.next_tetromino.2 {
            TradeState::NoTrade =>
                self.next_tetromino.2 = TradeState::Pending(target),
            TradeState::Pending(id) => {
                if id == target {
                    self.next_tetromino.2 = TradeState::NoTrade;
                }
                else {
                    self.next_tetromino.2 = TradeState::Pending(target);
                }
            },
            TradeState::Confirm(_) => (),            
        }
    }

    pub fn rotate_tetromino(&mut self) {
        self.tetromino.1 = (self.tetromino.1 + 1) % 4;
        if self.collision(0,0) {
            self.tetromino.1 = (self.tetromino.1 + 3) % 4;
        }
    }

    pub fn move_down(&mut self) {
        self.tetro_pos.0 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.0 += 1;
	    self.tetro_to_board();
	    self.clear_lines();
	    self.tetro_pos = (ROWS as i8 - 3, COLS as i8 / 2 - 1);
 	    self.new_tetromino();
	}
    }

    pub fn drop(&mut self) {
        while !self.collision(-1, 0) {
            self.tetro_pos.0 -= 1;
        }
    }

    fn clear_lines(&mut self) {
        let mut clear_line = true;
        for i in 0..ROWS {
	    clear_line = true;
	    for j in 0..COLS {
	        if self.board[ROWS - i - 1][j] == Cell::E {
		    clear_line = false;
		    break;
	        }
            }
	    if clear_line {
	    	self.delete_line(ROWS - i - 1);
	    }
        }
    }

    fn delete_line(&mut self, line: usize) {
        for i in line..(ROWS - 1) {
	    for j in 0..COLS {
	        self.board[i][j] = self.board[i + 1][j];
	    }
        }
    }

    pub fn move_right(&mut self) {
        self.tetro_pos.1 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.1 += 1;
	}
    }

    pub fn move_left(&mut self) {
        self.tetro_pos.1 += 1;

	if self.collision(0,0) {
	    self.tetro_pos.1 -= 1;
	}
    }

    fn tetro_to_board(&mut self) {
        for i in 0..4 {
	    for j in 0..4 {
	        if self.tetromino.0[self.tetromino.1][i][j] != 0 {
		    self.board[(i as i8 + self.tetro_pos.0) as usize]
                        [(j as i8 + self.tetro_pos.1) as usize] =
                        cell_of_shape(self.tetromino.0);
	        }
            }
        }
    }

    fn collision(&mut self, dr: i8, dc: i8) -> bool {
        let nr = self.tetro_pos.0 + dr;
        let nc = self.tetro_pos.1 + dc;

        for i in 0..4 {
	    for j in 0..4 {
 	        if self.tetromino.0[self.tetromino.1][i][j] != 0 {
		    if (i as i8 + nr) < ROWS as i8 && (i as i8 + nr) >= 0 &&
                        (j as i8 + nc) < COLS as i8 && (j as i8 + nc) >= 0 {
		            if self.board[(i as i8 + nr) as usize]
                                [(j as i8 + nc) as usize] != Cell::E {
			            return true;
		                }
                        }
		    else {
		        return true;
		    }
                }
            }
        }
        false
    }
}

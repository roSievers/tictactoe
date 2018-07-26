use coord;
use std::fmt;
use std::fmt::{Debug};
use std::ops::{Index, IndexMut};
use rand::random;

// TODO: Token<T> { Clear(T), .. }
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Token {
    Clear,
    Circle,
    Cross
}

#[derive(Clone, Copy)]
pub enum Player {
    Circle,
    Cross
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    Undecided,
    Circle,
    Cross,
    Draw
}

#[derive(Clone, Debug)]
pub struct Local {
    entries : [Token; 9],
    pub total : Ownership
}

#[derive(Clone, Debug)]
pub struct Global {
    entries : [Local; 9],
    total : Ownership
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Clear => write!(f, " "),
            Token::Circle => write!(f, "O"),
            Token::Cross => write!(f, "X")
        }
    }
}

impl Debug for Ownership {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ownership::Undecided => write!(f, " "),
            Ownership::Circle => write!(f, "O"),
            Ownership::Cross => write!(f, "X"),
            Ownership::Draw => write!(f, "-")
        }
    }
}

impl From<Ownership> for Token {
    fn from(owner: Ownership) -> Self {
        match owner {
            Ownership::Undecided => Token::Clear,
            Ownership::Circle => Token::Circle,
            Ownership::Cross => Token::Cross,
            Ownership::Draw => Token::Clear
        }
    }
}

impl Local {
    pub fn new() -> Self {
        let entries = [Token::Clear; 9];
        let total = Ownership::Undecided;
        Local { entries, total }
    }

    pub fn random() -> Self {
        let mut result = Self::new();
        for i in 0..9 {
            let token = match random() : bool {
                true => Token::Cross,
                false => Token::Circle
            };
            result.entries[i] = token;
        }
        result
    }

    fn place_token(&mut self, position: coord::Local, token: Token) {
        self[position] = token;

        // Check win conditions and set total Ownership
        // Check horizontal
        let row = 3 * position.get_y() as usize;
        let row_is_won = self.entries[row] == token && 
                         self.entries[row + 1] == token &&
                         self.entries[row + 2] == token;
        // Check vertical
        let col = position.get_x() as usize;
        let col_is_won = self.entries[col] == token && 
                         self.entries[3 + col] == token &&
                         self.entries[6 + col] == token;
        // Check diagonals
        let down_diag_is_won = self.entries[0] == token && 
                               self.entries[4] == token &&
                               self.entries[8] == token;
        let up_diag_is_won = self.entries[2] == token && 
                               self.entries[4] == token &&
                               self.entries[5] == token;

        // Set total ownership
        if row_is_won || col_is_won || down_diag_is_won || up_diag_is_won {
            self.total = token.into();
        }
        else
        {
            // Handle stalemate
            if coord::Local::iter().all( | pos | self[pos] != Token::Clear ) {
                self.total = Ownership::Draw
            } else {
                self.total = Ownership::Undecided
            }
        }

    }


}

impl Index<coord::Local> for Local {
    type Output = Token;

    fn index(&self, local_coord: coord::Local) -> &Self::Output {
        &self.entries[ local_coord.index() ]
    }
}

impl IndexMut<coord::Local> for Local {
    fn index_mut(&mut self, local_coord: coord::Local) -> &mut Self::Output {
        &mut self.entries[ local_coord.index() ]
    }
}

impl Global {
    pub fn new() -> Self {
        let entries = [ Local::new(), Local::new(), Local::new(), Local::new(),
                        Local::new(), Local::new(), Local::new(), Local::new(),
                        Local::new()];
        let total = Ownership::Undecided;
        Global { entries, total }
    }

    pub fn random() -> Self {
        let mut result = Self::new();
        for i in 0..9 {
            result.entries[i] = Local::random();
        }
        result
    }

    pub fn at(&mut self, global_coord: coord::Global) -> &mut Token {
        &mut self[global_coord.get_region()][global_coord.get_local()]
    }

    pub fn place_token(&mut self, position: coord::Global, token: Token) {
        let region : &mut Local = &mut self[position.get_region()];
        region.place_token(position.get_local(), token);
    }
}

impl Index<coord::Local> for Global {
    type Output = Local;

    fn index(&self, region_coord: coord::Local) -> &Self::Output {
        &self.entries[ region_coord.index() ]
    }
}

impl IndexMut<coord::Local> for Global {
    fn index_mut(&mut self, region_coord: coord::Local) -> &mut Self::Output {
        &mut self.entries[ region_coord.index() ]
    }
}

impl Index<coord::Global> for Global {
    type Output = Token;

    fn index(&self, global_coord: coord::Global) -> &Self::Output {
        &self[global_coord.get_region()][global_coord.get_local()]
    }
}

impl IndexMut<coord::Global> for Global {
    fn index_mut(&mut self, global_coord: coord::Global) -> &mut Self::Output {
        &mut self[global_coord.get_region()][global_coord.get_local()]
    }
}

impl From<Player> for Token {
    fn from(player: Player) -> Self {
        match player {
            Player::Circle => Token::Circle,
            Player::Cross => Token::Cross
        }
    }
}

impl From<Token> for Ownership {
    fn from(player: Token) -> Self {
        match player {
            Token::Circle => Ownership::Circle,
            Token::Cross => Ownership::Cross,
            Token::Clear => Ownership::Undecided
        }
    }
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle
        }
    }
}
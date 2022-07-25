// app info and app operation 

use std::fmt::Display;

// label for cell status and game result
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    // occupied / game winned by X 
    X, 
    // occupied / game winned by O
    O,
    // not occupied / game in progress
    Null,
}

// make status print-able
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Status::X => 'X',
            Status::O => 'O',
            Status::Null => ' '
        })
    }
}

// app info
pub struct App {
    // chess board size
    pub size: usize,
    // cursor location
    pub row: usize,
    pub column: usize,
    // chess board status
    pub matrix: Vec<Vec<Status>>,
    // winner of this game
    pub winner: Status,
    // next player
    pub now_player: Status,
}

impl App {
    // cursor operation
    pub fn up(&mut self){
        if self.row  == 0{
            self.row = self.size - 1;
        }else{
            self.row -= 1;
        }
    }
    
    pub fn down(&mut self){
        if self.row  == self.size - 1{
            self.row = 0;
        }else{
            self.row += 1;
        }
    }

    pub fn left(&mut self){
        if self.column  == 0{
            self.column = self.size - 1;
        }else{
            self.column -= 1;
        }
    }

    pub fn right(&mut self){
        if self.column  == self.size - 1{
            self.column = 0;
        }else{
            self.column += 1;
        }
    }
    
    // check if index is within the range
    fn visit_cell(& self, i: usize, j: usize) -> Option<Status>{
        if i < self.size && j < self.size{
            return Some(self.matrix[i][j]);
        }
        None
    }

    // check if there is a 5-in-a-row for one cell
    fn check_cell(& self, i: usize, j: usize) -> Status{
        if i > 3 && self.visit_cell(i-4, j).is_some(){
            if self.visit_cell(i-4, j).unwrap() == self.visit_cell(i-3, j).unwrap()
                && self.visit_cell(i-3, j).unwrap() == self.visit_cell(i-2, j).unwrap() 
                && self.visit_cell(i-2, j).unwrap() == self.visit_cell(i-1, j).unwrap() 
                && self.visit_cell(i-1, j).unwrap() == self.visit_cell(i, j).unwrap() 
                && self.visit_cell(i, j).unwrap() != Status::Null 
                {
                return self.visit_cell(i, j).unwrap();
            }
        }
        if j > 3 && self.visit_cell(i,j-4).is_some(){
            if self.visit_cell(i, j-4).unwrap() == self.visit_cell(i, j-3).unwrap()
                && self.visit_cell(i, j-3).unwrap() == self.visit_cell(i, j-2).unwrap() 
                && self.visit_cell(i, j-2).unwrap() == self.visit_cell(i, j-1).unwrap() 
                && self.visit_cell(i, j-1).unwrap() == self.visit_cell(i, j).unwrap() 
                && self.visit_cell(i, j).unwrap() != Status::Null 
                {
                return self.visit_cell(i, j).unwrap();
            }
        }
        if i > 3 && j > 3 && self.visit_cell(i-4, j-4).is_some(){
            if self.visit_cell(i-4, j-4).unwrap() == self.visit_cell(i-3, j-3).unwrap()
                && self.visit_cell(i-3, j-3).unwrap() == self.visit_cell(i-2, j-2).unwrap() 
                && self.visit_cell(i-2, j-2).unwrap() == self.visit_cell(i-1, j-1).unwrap() 
                && self.visit_cell(i-1, j-1).unwrap() == self.visit_cell(i, j).unwrap() 
                && self.visit_cell(i, j).unwrap() != Status::Null 
                {
                return self.visit_cell(i, j).unwrap();
            }
        }
        if j > 3 && self.visit_cell(i+4, j-4).is_some(){
            if self.visit_cell(i+4, j-4).unwrap() == self.visit_cell(i+3, j-3).unwrap()
                && self.visit_cell(i+3, j-3).unwrap() == self.visit_cell(i+2, j-2).unwrap() 
                && self.visit_cell(i+2, j-2).unwrap() == self.visit_cell(i+1, j-1).unwrap() 
                && self.visit_cell(i+1, j-1).unwrap() == self.visit_cell(i, j).unwrap() 
                && self.visit_cell(i, j).unwrap() != Status::Null 
                {
                return self.visit_cell(i, j).unwrap();
            }
        }
        Status::Null
    }
    
    // put down a chess to occupy a vacant cell
    pub fn register(&mut self){
        if let Status::Null = self.matrix[self.row as usize][self.column as usize]{
            match self.now_player{
                Status::O => {
                    self.matrix[self.row as usize][self.column as usize] = Status::O;
                    self.now_player = Status::X;
                },
                Status::X => {
                    self.matrix[self.row as usize][self.column as usize] = Status::X;
                    self.now_player = Status::O;
                },
                _ => ()
            }
        }
    }

    // check if this game ends
    pub fn check(&mut self){
        for i in 0..self.size{
            for j in 0..self.size{
                match self.check_cell(i, j) {
                    Status::Null => continue,
                    other => {
                        self.winner = other;
                        self.now_player = Status::Null;
                        return ;
                    },
                }
            }
        }
        
    }

    // new app instance
    pub fn new() -> App{
        let size = 10;
        App {
            size: size,
            row: 0,
            column: 0,
            matrix: vec![vec![Status::Null; size];size],
            winner: Status::Null,
            now_player: Status::X,
        }
    }
}

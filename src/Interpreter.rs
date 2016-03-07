use std::str::Chars;

pub trait StackValue {
    fn isNum(&self)->bool;
    fn isChar(&self)->bool;
}

pub struct Program {
    instructions: [[char; 80]; 25],
    stack: Vec<Box<StackValue>>,
    x: usize,
    y: usize,
    xDirection: usize,
    yDirection: usize
}

impl Program {
    pub fn new(program: &String) -> Program {
        let mut instructions = [[' '; 80]; 25];
        let mut stack = Vec::new();
        Program { 
            instructions: [[' '; 80]; 25],
            stack: stack,
            x:0,
            y:0,
            xDirection:1,
            yDirection:0
        }
    }
}

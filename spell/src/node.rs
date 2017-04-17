use std::collections::HashMap;
pub struct Node {
    value: Option<char>,
    times: usize,
    children: Option<Box<HashMap<char, Node>>>,
}

impl Node {
    pub fn new(character: Option<char>) -> Self{
        match character {
            None => Node {
                value: None,
                times: 0,
                children: None,
            },
            Some(_) => Node {
                value: character,
                times: 1,
                children: None,
            },
        }
    }
    



}

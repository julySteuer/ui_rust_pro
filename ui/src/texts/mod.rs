pub mod char;
pub mod text;
use text::Text;
use minifb::{Window, Key};
use crate::color::Color;

fn a() -> char::Char{
    let a_structure:[[u32;5];8] = [[0,0,0,0,0],
                               [0,0,0,0,0],// 2 row buffer up
                               [0,0,1,0,0],
                               [0,0,1,0,0],
                               [0,1,1,1,0],
                               [1,0,0,0,1],
                               [0,0,0,0,0],//2 row buffer down
                               [0,0,0,0,0]];
    let  a_char = 'A';
    char::Char {
        structure:a_structure,
        character: String::from(a_char),
        col: Color{r:255, g:255, b:255}
    }
}

impl Text { // add elements and render them
    pub fn add(&mut self, window:&Window){
        window.get_keys().iter().for_each(|key|
            match key {
                Key::A => self.characters.push(a()),
                _ => ()
            }
        );
    }

    pub fn new() -> Text {
        Text {
            characters: vec![]
        }
    }

    pub fn render(&self, window:&mut Vec<u32>){
        for i in &self.characters {
            i.render(window, 100);
        }
    }
}
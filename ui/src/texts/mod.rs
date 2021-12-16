pub mod char;
pub mod text;
use text::Text;
use minifb::{Window, Key};
use crate::color::Color;
use crate::context::Context; //macro rules for making char

/*
[[0,0,0,0,0],
[0,0,0,0,0],// 2 row buffer up
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],//2 row buffer down
[0,0,0,0,0]]
*/
macro_rules! make_letter {
    ($name:ident, $structure:expr) => {
        fn $name(context:&Context) -> char::Char{
            let structure:[[u32;5];9] = $structure;
            let character = stringify!($name);
            char::Char {
                structure,
                character: String::from(character),
                col: Color{r:255, g:255, b:255},
                context: context.clone()
            }
        }
    };
}


make_letter!(b, [[0,0,0,0,0],
    [0,0,0,0,0],// 2 row buffer up
    [1,1,1,0,0],
    [1,0,0,1,0],
    [1,1,1,1,0],
    [1,0,0,1,0],
    [1,1,1,0,0],//2 row buffer down
    [0,0,0,0,0],
    [0,0,0,0,0]]);

impl Text { // add elements and render them 
    pub fn add(&mut self, window:&Window){
        if window.get_keys().len() != 0 {
            self.local_context.x += 6;
        }
        window.get_keys().iter().for_each(|key|
            match key {
                Key::B => self.characters.push(b(&self.local_context)),
                Key::Backspace => {self.characters.pop();self.local_context.x -= 12;},
                _ => ()
            }
        );
    }
}
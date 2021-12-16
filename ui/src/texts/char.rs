use minifb::Window;
use crate::color::{Color, rgb_2_int};
use crate::context;

#[derive(Debug)]
pub struct Char {
    pub character: String,
    pub structure: [[u32; 5];9],
    pub col: Color,
    pub context: context::Context
}

impl Char { // Uppercase and lowercase
    fn new(character:String, structure:[[u32; 5];9], col:Color, context: context::Context) -> Char{
        Char {
            character,
            structure,
            col,
            context
        }
    }
    /*
    fn resize(&self) -> Vec<Vec<u32>> {
        for y in 0..9 {
            for x in 0..5 {
                
            }
        }
    }
    */

    pub fn render(&self, buffer:&mut Vec<u32>, width:u32){
        for y in 0..9 {
            for x in 0..5 {
                if self.structure[y][x] == 1 {
                    let added = width as usize * (y + self.context.y as usize) + (x + self.context.x as usize);
                    buffer[added] = rgb_2_int(&self.col);
                }
            } 
        }
    }
}
use minifb::Window;
use crate::color::{Color, rgb_2_int};

pub struct Char {
    pub character: String,
    pub structure: [[u32; 5];8],
    pub col: Color
}

impl Char { // Uppercase and lowercase
    fn new(character:String, structure:[[u32; 5];8], col:Color) -> Char{
        Char {
            character,
            structure,
            col
        }
    }

    pub fn render(&self, buffer:&mut Vec<u32>, width:u32){
        for y in 0..8 {
            for x in 0..5 {
                if self.structure[y][x] == 1 {
                    println!("Here");
                    let added = width as usize * y + x;
                    println!("Added: {}", added);
                    buffer[added] = rgb_2_int(&self.col);
                }
            } 
        }
    }
}
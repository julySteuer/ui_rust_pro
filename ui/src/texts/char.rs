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
    fn resize(&self) -> Vec<Vec<u32>> {
        let mut out: Vec<Vec<u32>> = vec![vec![0;(self.context.scale + 5) as usize];(self.context.scale + 8) as usize];
        for y in 0..9 { // move 0 chars effectivly
            for x in 0..5 {
                if self.structure[y][x] == 1 {
                    for zoom_x in 0..self.context.scale {
                        for zoom_y in 0..self.context.scale {
                            out[y+zoom_y as usize][x+zoom_x as usize] = 1;
                        }
                    }
                }
                else {
                    for zoom_x in 0..self.context.scale {
                        for zoom_y in 0..self.context.scale {
                            out[y+zoom_y as usize][x+zoom_x as usize] = 0;
                        }
                    }
                }
            }
        }
        out
    }

    pub fn render(&self, buffer:&mut Vec<u32>, width:u32){
        let char_buffer = self.resize();
        for y in 0..char_buffer.len() {
            for x in 0..char_buffer[0].len() {
                if char_buffer[y][x] == 1 {
                    let added = width as usize * (y + self.context.y as usize) + (x + self.context.x as usize);
                    buffer[added] = rgb_2_int(&self.col);
                }
            } 
        }
    }
}
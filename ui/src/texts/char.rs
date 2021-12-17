use minifb::Window;
use crate::color::{Color, rgb_2_int};
use crate::context;

#[derive(Debug)]
pub struct Char {
    pub character: String,
    pub structure: [[u32; 5];9],
    pub col: Color,
    pub context: context::Context,
    pub out:Vec<Vec<u32>>
}

impl Char { // Uppercase and lowercase
    fn new(character:String, structure:[[u32; 5];9], col:Color, context: context::Context) -> Char{
        Char {
            character,
            structure,
            col,
            context,
            out:vec![vec![]]
        }
    }
    pub fn resize(&self) -> Vec<Vec<u32>> {
        let mut out: Vec<Vec<u32>> = vec![vec![0;(self.context.scale * 5) as usize];(self.context.scale * 9) as usize];
        for y in 0..9 {
            for x in 0..5 {
              for z_x in 0..self.context.scale {
                  for z_y in 0..self.context.scale {
                    out[(y*self.context.scale+z_y) as usize][(x*self.context.scale+z_x) as usize] = self.structure[y as usize][x as usize];
                  }
              }  
            }
        }
        out
    }

    pub fn render(&self, buffer:&mut Vec<u32>, width:u32){
        let char_buffer = &self.out;
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
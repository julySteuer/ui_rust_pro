#[derive(Debug, Clone)]
pub struct Context {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub scale:u32
}

impl Context {
    pub fn new(x:u32, y:u32, width:u32, height:u32, scale:u32) -> Context{
        Context {
            x, 
            y,
            width,
            height,
            scale
        }
    }
}
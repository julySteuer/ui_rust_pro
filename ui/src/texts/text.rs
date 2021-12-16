use crate::texts::char::Char;
use minifb::{Window, Key};
use crate::context::Context;

pub struct Text {
    pub characters: Vec<Char>,
    pub global_context:Context,
    pub local_context:Context
}

impl Text {
    pub fn new(context: Context) -> Text {
        Text {
            characters: vec![],
            global_context: context.clone(),
            local_context: context.clone()
        }
    }

    pub fn render(&self, window:&mut Vec<u32>){
        for i in &self.characters {
            i.render(window, 640);
        }
    }
}
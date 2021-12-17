pub mod char;
pub mod text;
use text::Text;
use minifb::{Window, Key};
use crate::color::Color;
use crate::context::Context; //macro rules for making char

/*
[[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0],
[0,0,0,0,0]]
*/
macro_rules! make_letter {
    ($name:ident, $structure:expr) => {
        fn $name(context:&Context) -> char::Char{
            let structure:[[u32;5];9] = $structure;
            let character = stringify!($name);
            let mut c = char::Char {
                structure,
                character: String::from(character),
                col: Color{r:255, g:255, b:255},
                context: context.clone(),
                out: vec![vec![]]
            };
            c.out = c.resize();
            c
        }
    };
}

make_letter!(a, [[0,0,0,0,0],
    [0,0,0,0,0],// 2 row buffer up
    [0,0,1,0,0],
    [0,1,0,1,0],
    [1,1,1,1,1],
    [1,0,0,0,1],
    [1,0,0,0,1],//2 row buffer down
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(b, [[0,0,0,0,0],
    [0,0,0,0,0],// 2 row buffer up
    [1,1,1,0,0],
    [1,0,0,1,0],
    [1,1,1,1,0],
    [1,0,0,1,0],
    [1,1,1,0,0],//2 row buffer down
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(c, [[0,0,0,0,0],
    [0,0,0,0,0],// 2 row buffer up
    [0,1,1,1,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [0,1,1,1,0],//2 row buffer down
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(d, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,1,1,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(e, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,1,1,1,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(f, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(g, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,0,1,1,1],
    [1,0,0,0,1],
    [1,1,1,1,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(h, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,1,1,1,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(i, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(j, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,1],
    [0,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,1,1,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(k, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,1,0],
    [1,0,1,0,0],
    [1,1,0,0,0],
    [1,0,1,0,0],
    [1,0,0,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(l, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [1,1,1,1,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(m, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,1,0,1,1],
    [1,0,1,0,1],
    [1,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(n, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,1,0,0,1],
    [1,0,1,0,1],
    [1,0,0,1,1],
    [1,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(o, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,1,1,1,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,1,1,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(p, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,1],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(q, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,1,1,1,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,1,1,1,1],
    [0,0,0,0,1],
    [0,0,0,0,0]]);

make_letter!(r, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,0],
    [1,0,0,1,0],
    [1,1,1,1,0],
    [1,0,1,0,0],
    [1,0,0,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(s, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [1,0,0,0,0],
    [1,1,1,1,1],
    [0,0,0,0,1],
    [1,1,1,1,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(t, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,1,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(u, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,1,1,1,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(v, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [0,1,0,1,0],
    [0,0,1,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(w, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [1,0,0,0,1],
    [1,0,1,0,1],
    [1,1,0,1,1],
    [1,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(x, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [0,1,0,1,0],
    [0,0,1,0,0],
    [0,1,0,1,0],
    [1,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(y, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,0,0,0,1],
    [0,1,0,1,0],
    [0,0,1,0,0],
    [0,0,0,1,0],
    [0,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(z, [[0,0,0,0,0],
    [0,0,0,0,0],
    [1,1,1,1,1],
    [0,0,0,1,0],
    [0,0,1,0,0],
    [0,1,0,0,0],
    [1,1,1,1,1],
    [0,0,0,0,0],
    [0,0,0,0,0]]);

make_letter!(space, [[0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,0]]);
impl Text { // add elements and render them 
    pub fn add(&mut self, window:&Window){
        if window.get_keys().len() != 0 {
            self.local_context.x += self.local_context.scale * 5;
        }
        window.get_keys().iter().for_each(|key|
            match key {
                Key::A => self.characters.push(a(&self.local_context)),
                Key::B => self.characters.push(b(&self.local_context)),
                Key::C => self.characters.push(c(&self.local_context)),
                Key::D => self.characters.push(d(&self.local_context)),
                Key::E => self.characters.push(e(&self.local_context)),
                Key::F => self.characters.push(f(&self.local_context)),
                Key::G => self.characters.push(g(&self.local_context)),
                Key::H => self.characters.push(h(&self.local_context)),
                Key::I => self.characters.push(i(&self.local_context)),
                Key::J => self.characters.push(j(&self.local_context)),
                Key::K => self.characters.push(k(&self.local_context)),
                Key::L => self.characters.push(l(&self.local_context)),
                Key::M => self.characters.push(m(&self.local_context)),
                Key::N => self.characters.push(n(&self.local_context)),
                Key::O => self.characters.push(o(&self.local_context)),
                Key::P => self.characters.push(p(&self.local_context)),
                Key::Q => self.characters.push(q(&self.local_context)),
                Key::R => self.characters.push(r(&self.local_context)),
                Key::S => self.characters.push(s(&self.local_context)),
                Key::T => self.characters.push(t(&self.local_context)),
                Key::U => self.characters.push(u(&self.local_context)),
                Key::V => self.characters.push(v(&self.local_context)),
                Key::W => self.characters.push(w(&self.local_context)),
                Key::X => self.characters.push(x(&self.local_context)),
                Key::Y => self.characters.push(y(&self.local_context)),
                Key::Z => self.characters.push(z(&self.local_context)),
                Key::Backspace => {self.characters.pop();self.local_context.x -= (self.local_context.scale*5)*2;},
                Key::Space => self.characters.push(space(&self.local_context)),
                _ => ()
            }
        );
    }
}
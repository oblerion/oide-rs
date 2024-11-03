extern crate raylib;
use raylib::prelude::*;

pub fn lib_draw(mut d:RaylibDrawHandle){
    d.clear_background(Color::GREEN);
    d.draw_text("start", 12, 12, 20, Color::BLACK)
}

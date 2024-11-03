extern crate raylib;
use raylib::prelude::*;
//mod rl2;
//use rl2::*;

pub struct SOide
{
    pub font:Font
}

impl SOide
{
    // pub fn load_font(
    //     mut self,
    //     mut rl:RaylibHandle,
    //     thread:&RaylibThread,
    //     path:&str)
    // {
        
    //     let ef = rl.load_font(thread,path);
    //     if ef.is_ok()
    //     {
    //         self.font=ef.unwrap();
    //     }
    //     else {
    //         //self.font=rl.get_font_default().to_raw()
    //     }

    // }
    pub fn print(&self,
        mut d:RaylibDrawHandle,
        text:&str)
    {
        let v = Vector2{x:12.0,y:12.0};
        let lfont = &self.font;
        d.draw_text_ex(lfont, text,v, 12.0, 0.0, Color::BLACK);
    }
}
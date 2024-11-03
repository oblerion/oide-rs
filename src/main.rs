extern crate raylib;
use raylib::prelude::*;
//mod lib;
mod rl2;
mod oide;


fn main() {
    let mut rl2= 
        rl2::init_window(1000,600,"hello");
    //rl2.set_target_fps(60);

    let oide = oide::SOide{ 
        font:rl2.load_font("./asset/tic-80.ttf")
    };
    while !rl2.window_should_close()
    {
        let k = &format!("{:?}",rl2.get_key_pressed());
        let mut d = rl2.begin_drawing();
            d.clear_background(Color::AZURE);
            d.draw_rectangle(32, 32, 20, 20, Color::BLACK);
            oide.print(d, k);
            
    }

}

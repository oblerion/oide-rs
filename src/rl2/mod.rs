extern crate raylib;
use raylib::prelude::*;

pub struct Rl2
{
    rl:RaylibHandle,
    thread:RaylibThread
}

pub fn init_window(width:i32,height:i32,text:&str) -> Rl2
{
    let mut bld = raylib::init();
    let (rl, thread) = bld
        .size(width, height)
        .title(text)
        .build();
    let mut rl2 = Rl2 { rl: rl, thread: thread };
    rl2.set_target_fps(60);
    return rl2;
}

#[allow(dead_code)]
impl Rl2
{
// window
    pub fn set_window_title(&mut self,text:&str)
    {
        self.rl.set_window_title(&self.thread,text);
    }
    pub fn set_window_size(&mut self, width:i32, height:i32)
    {
        self.rl.set_window_size(width, height);
    }
 
    pub fn window_should_close(&self) -> bool
    {
        return self.rl.window_should_close();
    }
// font load
    pub fn load_font(&mut self,path:&str) -> Font
    {
        return self.rl.load_font(&self.thread, path).unwrap();
        //return rf.unwrap();
    }
// font unload
    pub fn unload_font(&mut self,font:Font)
    {
        self.rl.unload_font(font.make_weak());
    }
// image load
    pub fn load_texture(&mut self,path:&str) -> Texture2D
    {
        return self.rl.load_texture(&self.thread, path).unwrap();
    }
// other
    pub fn set_target_fps(&mut self,fps:u32)
    {
        self.rl.set_target_fps(fps);
        
    }
// input
    pub fn get_key_pressed(&mut self) -> Option<crate::consts::KeyboardKey>
    {
        return self.rl.get_key_pressed();
    }
// drawing
    pub fn begin_drawing(&mut self) -> RaylibDrawHandle
    {
        return self.rl.begin_drawing(&self.thread);
    }

} 


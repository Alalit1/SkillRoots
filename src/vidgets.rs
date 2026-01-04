use fltk::{button::Button, frame::Frame, group::Group, prelude::*, window::Window};
use fltk::enums::Color;



struct Button{
    pub position_x: u16,
    pub position_y: u16,
    pub height: u16,
    pub width: u16,
    pub text: str,
}
impl Button{
    pub fn new(position_x: u16, position_y: u16,width: u16, height: u16, text: str) -> Self {
        Self { 
            position_x,
            position_y,
            width, 
            height,
            text, 
        }
    }
    pub fn output(&self){
        self.button = Button::new(self.position_x, self.position_y, self.width, self.height, self.text);
    }
    pub fn callback(&self,command){
        self.button.set_callback(command);
    }
}

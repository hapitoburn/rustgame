use std::collections::HashMap;
use sdl2::render::Texture;

pub struct Resource<'a>{
    pub map : HashMap<&'static str, Texture<'a>>,
}
impl <'a> Resource<'a>{
    pub fn get(&mut self, key : &str) -> Option<&Texture>{
        return self.map.get(key);
    }
}

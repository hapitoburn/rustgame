extern crate sdl2;
mod resources;

use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::mouse::MouseButton;

struct Button{
    x : i32,
    y : i32,
    w : u32,
    h : u32,
    image : &'static str
}
impl Button{
    fn show(&mut self, can: &mut sdl2::render::Canvas<sdl2::video::Window>, texture : Option<&Texture>){
        if let Some(t) = texture{
            can.copy(t, None, self.rect()).expect("Render failed");
        }
    }
    fn rect(&self) -> Rect{
        return Rect::new(self.x, self.y, self.w, self.h);
    }
    fn on_click(&mut self, events : &sdl2::EventPump, f : &Fn()){
        if events.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
            let state = events.mouse_state();
            f();
            println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("images.png").unwrap();

    let mut rc = resources::Resource{map : HashMap::new()};
    rc.map.insert("images.png", texture);

    let mut bx = Button{ x : 0, y : 0, w : 64, h : 64, image : "images.png"};
    bx.show(&mut canvas, rc.get(bx.image));

    canvas.present();
    let mut events = sdl_context.event_pump().unwrap();
    let mut prev_keys = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Create a set of pressed Keys.
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        }

        prev_keys = keys;

        bx.on_click(&events, &|| println!("tada") );

        std::thread::sleep(Duration::from_millis(100));
    }

}

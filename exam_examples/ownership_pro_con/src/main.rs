extern crate sdl2;

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;

fn main() -> Result<(), String> {
    let _image_context = sdl2::image::init(::sdl2::image::INIT_PNG)?;

    let sdl_context = sdl2::init()?;
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::WindowCanvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump()?;


    let texture_creator = canvas.texture_creator();
    let mut cache = TextureCache::new(texture_creator);


    let mut s = 1;
    let mut x = 0;

    loop {
        x += s * 2;
        if x >= 400 {
            s = -1;
        } else if x <= 0 {
            s = 1;
        }


        canvas.clear();
        canvas.copy(cache.get_or_load(MyTexture::Crab)?, None, Some(Rect::new(x, 320, 240, 160)))?;
        canvas.present();

        // abort if close is requested
        for event in event_pump.poll_iter() {
            if let Event::Window {
                timestamp: _,
                window_id: _,
                win_event: WindowEvent::Close
            } = event {
                return Ok(());
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum MyTexture {
    Crab,
}

impl MyTexture {
    fn path(&self) -> &str {
        match self {
            &MyTexture::Crab => "rustacean-flat-happy.png"
        }
    }
}

struct TextureCache<'r, T> {
    creator: TextureCreator<T>,
    cached: HashMap<MyTexture, Texture<'r>>,
}

impl<'r, T> TextureCache<'r, T> {
    fn new(creator: TextureCreator<T>) -> TextureCache<'r, T> {
        TextureCache {
            creator,
            cached: Default::default(),
        }
    }

    fn get_or_load(&'r mut self, id: MyTexture) -> Result<&Texture<'r>, String> {
        if !self.cached.contains_key(&id) {
            let texture = self.creator.load_texture(id.path())?;
            self.cached.insert(id, texture);
        }

        self.cached
            .get(&id)
            .ok_or("Insert failed".into())
    }
/*
    fn load(&mut self, id: MyTexture) -> Result<(), String> {
        self.cached.insert(id, self.creator.load_texture(id.path())?);
        Ok(())
    }

    fn get(&self, id: &MyTexture) -> Option<&Texture<'r>> {
        self.cached.get(id)
    }
    */
}
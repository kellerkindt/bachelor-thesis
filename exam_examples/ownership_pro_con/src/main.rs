extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

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

    let _image_context = sdl2::image::init(::sdl2::image::INIT_PNG)?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("rustacean-flat-happy.png")?;

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
        canvas.copy(&texture, None, Some(Rect::new(x, 320, 240, 160)))?;
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

extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let mut window = video.window("Wydy Client", 800, 600)
        .borderless()
        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();


    let mut event_pump = context.event_pump().unwrap();
    {
        let mut surface = window.surface_mut(&event_pump).unwrap();
        surface.set_alpha_mod(0);
    }
    let mut renderer = window.renderer().build().unwrap();

    'running: loop {
        renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
        renderer.clear();
        renderer.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
    }
}

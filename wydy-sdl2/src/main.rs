extern crate sdl2;
extern crate sdl2_ttf;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_ttf::Font;
use std::path::Path;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 100;

fn main() {
    let context = sdl2::init().unwrap();
    let ttf = sdl2_ttf::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("Wydy Client", SCREEN_WIDTH, SCREEN_HEIGHT)
        .borderless()
//        .fullscreen_desktop()
        .opengl()
        .build()
        .unwrap();


    let mut event_pump = context.event_pump().unwrap();
    let mut renderer = window.renderer().build().unwrap();
    let mut text = Text::new("".to_string(), &ttf, &mut renderer);
    renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
    renderer.clear();
    renderer.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::TextInput { text: t, .. } => {
                    let mut s = text.text().to_string();
                    s.push_str(&t);
                    text.change_text(s, &mut renderer);
                    println!("{}", text.text());
                }
                Event::Window { win_event_id, .. } => {
                    match win_event_id {
                        WindowEventId::FocusLost => {
                            renderer.window_mut().unwrap().raise();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        std::thread::sleep_ms(50);
    }
}

struct Text<'a> {
    text: String,
    font: Font<'a>,
    texture: Texture,
}

impl<'a> Text<'a> {
    pub fn new(text: String,
               ttf_content: &'a sdl2_ttf::Sdl2TtfContext,
               renderer: &mut Renderer)
               -> Text<'a> {
        let font =
            ttf_content.load_font(Path::new("/usr/share/fonts/fira-mono/FiraMono-Medium.otf"),
                           24)
                .unwrap();
        let text_tmp = if text.is_empty() {
            " ".to_string()
        } else {
            text.clone()
        };
        let surface = font.render(&text_tmp).blended(Color::RGBA(255, 255, 255, 255)).unwrap();

        Text {
            text: text,
            font: font,
            texture: renderer.create_texture_from_surface(&surface).unwrap(),
        }
    }

    pub fn change_text(&mut self, new_string: String, renderer: &mut Renderer) {
        let text_tmp = if new_string.is_empty() {
            " ".to_string()
        } else {
            new_string.clone()
        };
        self.text = new_string;
        let surface = self.font.render(&text_tmp).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        self.texture = renderer.create_texture_from_surface(&surface).unwrap();
        renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
        renderer.clear();
        let target = Rect::new(10, 25, self.width(50), 50);
        renderer.copy(self.texture(), None, Some(target)).unwrap();
        renderer.present();
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn texture(&mut self) -> &mut Texture {
        &mut self.texture
    }

    pub fn width(&self, height: u32) -> u32 {
        self.text.len() as u32 * (height / 3)
    }
}

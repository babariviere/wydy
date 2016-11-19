extern crate sdl2;
extern crate sdl2_ttf;
extern crate wydy;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_ttf::Font;
use std::time::Duration;
use std::path::Path;
use std::thread;
use wydy::client;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 100;

fn main() {
    let context = sdl2::init().unwrap();
    let ttf = sdl2_ttf::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("Wydy Client", SCREEN_WIDTH, SCREEN_HEIGHT)
        .borderless()
        .opengl()
        .build()
        .unwrap();


    let mut event_pump = context.event_pump().unwrap();
    let mut renderer = window.renderer().build().unwrap();
    let mut text = Text::new("".to_string(), &ttf, &mut renderer);
    let mut server = client::connect_to_server("127.0.0.1:9654").unwrap();
    renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
    renderer.clear();
    renderer.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    text.pop(&mut renderer);
                }
                Event::KeyDown { keycode, .. } => {
                    let keycode = keycode;
                    match keycode {
                        Some(Keycode::Escape) => {
                            client::send_command(&mut server, "cancel", false).unwrap();
                            client::command_response(&mut server).unwrap();
                            break 'running;
                        }
                        Some(Keycode::Backspace) => text.pop(&mut renderer),
                        Some(Keycode::Return) => {
                            let txt = text.text();
                            println!("{}", txt);
                            client::send_command(&mut server, txt, false).unwrap();
                            client::command_response(&mut server).unwrap();
                            break 'running;
                        }
                        _ => {}
                    }
                }
                Event::TextInput { text: t, .. } => text.push_str(&t, &mut renderer),
                Event::Window { win_event_id: WindowEventId::FocusLost, .. } => {
                    // Keep keyboard focus
                    renderer.window_mut().unwrap().raise();
                }

                e => {
                    println!("{:?}", e);
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

struct Text<'a> {
    text: String,
    font: Font<'a>,
    texture: Option<Texture>,
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
            texture: Some(renderer.create_texture_from_surface(&surface).unwrap()),
        }
    }

    pub fn push_str(&mut self, new_text: &str, renderer: &mut Renderer) {
        self.text.push_str(new_text);
        self.update_texture(renderer);
        renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
        renderer.clear();
        self.draw(renderer);
        renderer.present();
    }

    pub fn pop(&mut self, renderer: &mut Renderer) {
        self.text.pop();
        self.update_texture(renderer);
        renderer.set_draw_color(Color::RGBA(100, 100, 100, 0));
        renderer.clear();
        self.draw(renderer);
        renderer.present();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        if self.text.is_empty() || self.texture.is_none() {
            return;
        }
        let width = self.width(50);
        let x = if width > SCREEN_WIDTH {
            SCREEN_WIDTH as i32 - width as i32 - 10
        } else {
            10
        };
        println!("{} {}", x, width);
        let target = Rect::new(x, 25, width, 50);
        renderer.copy(self.texture.as_ref().unwrap(), None, Some(target)).unwrap();
    }

    pub fn update_texture(&mut self, renderer: &mut Renderer) {
        if self.text.is_empty() {
            self.texture = None;
            return;
        }
        let surface =
            self.font.render(&self.text).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        self.texture = Some(renderer.create_texture_from_surface(&surface).unwrap());
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn width(&self, height: u32) -> u32 {
        self.text.len() as u32 * (height / 3 + 5)
    }
}

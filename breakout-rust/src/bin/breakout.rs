use breakout_rust::breakout::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

fn main() {
    let width = 1440;
    let height = 1080;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Breakout.Rust", width, height)
        .resizable()
        .opengl()
        .build()
        .unwrap();
    let mut can = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Error on canvas");
    let tc = can.texture_creator();
    let mut texture = tc
        .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .map_err(|e| e.to_string())
        .expect("Error on texture create");
    let mut e = sdl.event_pump().unwrap();

    let mut breakout = Breakout::new();
    breakout.new_game();

    'main: loop {
        for _event in e.poll_iter() {
            match _event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }
        can.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        can.clear();
        //can.copy(&texture, None, Some(Rect::new(0, 0, width, height))).expect("on copy");

        for x in 0..TILE_W {
            for y in 0..TILE_H {
                let xpos = x * 32;
                let ypos = y * 16;
                let color = Grid::color_from_type(&breakout.grid.blocks[x][y]);
                can.set_draw_color(color);
                can.draw_rect(sdl2::rect::Rect::new(xpos as i32, ypos as i32, 32, 16)).expect("draw rect");
            }
        }
        can.present();
    }
}

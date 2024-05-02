use breakout_rust::breakout::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

fn printtext(
    can: &mut sdl2::render::Canvas<sdl2::video::Window>,
    tex: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &sdl2::ttf::Font,
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    text: &str,
) {
    let text_surf = font.render(text).blended(color).unwrap();
    let text_surf_tex = tex.create_texture_from_surface(&text_surf).unwrap();
    let TextureQuery {
        width: wi,
        height: hi,
        ..
    } = text_surf_tex.query();
    can.copy(
        &text_surf_tex,
        Some(Rect::new(0, 0, wi, hi)),
        Some(Rect::new(x, y, wi, hi)),
    )
    .expect("on font copy");
}

fn main() {
    let width = 1440;
    let height = 1080;
    let sdl = sdl2::init().unwrap();
    let timer_subsystem = sdl.timer().unwrap();
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("./font.ttf", 32).unwrap();
    let tc = can.texture_creator();
    let _texture = tc
        .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
        .map_err(|e| e.to_string())
        .expect("Error on texture create");
    let mut e = sdl.event_pump().unwrap();
    let intro_surface = sdl2::surface::Surface::load_bmp("./logo.bmp").unwrap();
    let intro_texture = tc.create_texture_from_surface(intro_surface).unwrap();
    let mut breakout = Breakout::new();
    breakout.new_game();
    let mut ticks = timer_subsystem.ticks();
    let mut total_tick: u32 = 0;
    let mut intro = true;
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


        let tick = timer_subsystem.ticks();
        let tick_diff = tick-ticks;
        ticks = tick;
        total_tick += tick_diff;

        if intro {
            can.copy(&intro_texture, None, None).expect("on copy");
        } else {
            for x in 0..TILE_W {
                for y in 0..TILE_H {
                    let xpos = x * 32;
                    let ypos = y * 16;
                    let color = breakout.grid.color_from_type(&breakout.grid.blocks[x][y]);
                    can.set_draw_color(color);
                    can.draw_rect(sdl2::rect::Rect::new(xpos as i32, ypos as i32, 32, 16))
                        .expect("draw rect");
                }
            }

            let xpos = breakout.paddle.x;
            let ypos = breakout.paddle.y;
            can.set_draw_color(breakout.paddle.color);
            can.fill_rect(sdl2::rect::Rect::new(xpos, ypos, 200, 20))
                .expect("on fill");
            let xpos = breakout.ball.x;
            let ypos = breakout.ball.y;
            can.fill_rect(sdl2::rect::Rect::new(xpos, ypos, 16, 16))
                .expect("on ball");

            printtext(
                &mut can,
                &tc,
                &font,
                75,
                75,
                sdl2::pixels::Color::RGB(255, 255, 255),
                &format!("Lives: {} Score: {}", breakout.lives, breakout.score),
            );
        }
        can.present();       

        if intro {
            if total_tick > 2500 {
                total_tick = 0;
                intro = false;
            }
        } else if total_tick > 10 {
            total_tick  = 0;
            breakout.update();
            
            let keyboard_state = e.keyboard_state();
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
                breakout.paddle.move_left();
            }
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
                breakout.paddle.move_right();
            }
        }
    }
}

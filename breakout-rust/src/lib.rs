pub mod breakout {
    use rand::Rng;
    use sdl2::pixels::Color;
    
    pub const SCREEN_WIDTH: i32 = 1440;
    pub const SCREEN_HEIGHT: i32 = 1080;
    pub const TILE_W: usize = SCREEN_WIDTH as usize / 32;
    pub const TILE_H: usize = SCREEN_HEIGHT as usize / 2 / 16;
    pub const PADDLE_WIDTH: i32 = 200;
    pub const PADDLE_HEIGHT: i32 = 20;
    pub const BALL_SIZE: i32 = 16;
    pub const BALL_SPEED: i32 = 5;

    pub struct Paddle {
        pub x: i32,
        pub y: i32,
        pub color: Color,
    }

    impl Paddle {
        pub fn new(xpos: i32, ypos: i32) -> Paddle {
            Paddle {
                x: xpos,
                y: ypos,
                color: Color::RGB(255, 255, 255),
            }
        }

        pub fn move_left(&mut self) {
            if self.x > 0 {
                self.x -= 10;
            }
        }

        pub fn move_right(&mut self) {
            if self.x + PADDLE_WIDTH < SCREEN_WIDTH {
                self.x += 10;
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Block {
        pub color_type: u32,
    }

    pub struct Grid {
        pub blocks: Box<[[Block; TILE_H]; TILE_W]>,
    }

    impl Grid {
        pub fn new() -> Grid {
            let b= Box::new([[Block { color_type: 0 }; TILE_H]; TILE_W]);
            Grid { blocks: b }
        }

        pub fn fill_rand(&mut self) {
            let mut rng = rand::thread_rng();
            for col in self.blocks.iter_mut() {
                for block in col.iter_mut() {
                    block.color_type = rng.gen_range(1..8);
                }
            }
        }

        pub fn color_from_type(b: &Block) -> sdl2::pixels::Color {
            match b.color_type {
                1 => {
                    return sdl2::pixels::Color::RGB(255, 0, 0);
                }

                2 => {
                    return sdl2::pixels::Color::RGB(255, 255, 0);
                }

                3 => {
                    return sdl2::pixels::Color::RGB(255, 0, 25);
                }

                4 => {
                    return sdl2::pixels::Color::RGB(255, 255, 255);
                }

                5 => {
                    return sdl2::pixels::Color::RGB(255, 100, 50);
                }

                6 => {
                    return sdl2::pixels::Color::RGB(255, 200, 100);
                }

                7 => {
                    return sdl2::pixels::Color::RGB(255, 75, 125);
                }
                8 => {
                    return sdl2::pixels::Color::RGB(255, 60, 20);
                }
                _ => {
                    return sdl2::pixels::Color::RGB(0, 0, 0);
                }
            }
        }
    }

    pub struct Ball {
        pub x: i32,
        pub y: i32,
        pub dx: i32,
        pub dy: i32,
    }

    impl Ball {
        pub fn new() -> Ball {
            Ball {
                x: SCREEN_WIDTH / 2,
                y: SCREEN_HEIGHT / 2,
                dx: BALL_SPEED,
                dy: -BALL_SPEED,
            }
        }

        pub fn reset(&mut self) {
            self.x = SCREEN_WIDTH / 2;
            self.y = SCREEN_HEIGHT / 2 - 50;
            let mut rng = rand::thread_rng();
            self.dx = if rng.gen::<bool>() {
                BALL_SPEED
            } else {
                -BALL_SPEED
            };
            self.dy = -BALL_SPEED;
        }

        pub fn update(&mut self, paddle: &Paddle, grid: &mut Grid, score: &mut u32, lives: &mut u32) {
            self.x += self.dx;
            self.y += self.dy;

            if self.x <= 0 || self.x + BALL_SIZE >= SCREEN_WIDTH {
                self.dx = -self.dx;
            }

            if self.y <= 0 {
                self.dy = -self.dy;
            }

            if self.y + BALL_SIZE >= paddle.y
                && self.x >= paddle.x
                && self.x <= paddle.x + PADDLE_WIDTH
            {
                self.dy = -self.dy;
            } else if self.y + BALL_SIZE > SCREEN_HEIGHT {
                std::thread::sleep(std::time::Duration::from_secs(1));
                *lives -= 1;
                self.reset();
            }

            let ball_left = self.x;
            let ball_right = self.x + BALL_SIZE;
            let ball_top = self.y;
            let ball_bottom = self.y + BALL_SIZE;

            for col in 0..TILE_W {
                for row in 0..TILE_H {
                    if grid.blocks[col][row].color_type == 0 { continue; }
                    let block_x = col as i32 * 32;
                    let block_y = row as i32 * 16;
                    let block_right = block_x + 32;
                    let block_bottom = block_y + 16;
                    if ball_right > block_x
                        && ball_left < block_right
                        && ball_bottom > block_y
                        && ball_top < block_bottom
                    {
                        self.dy = -self.dy;
                        grid.blocks[col][row].color_type = 0;
                        *score += 10;
                    }
                }
            }
        }
    }

    pub struct Breakout {
        pub paddle: Paddle,
        pub ball: Ball,
        pub score: u32,
        pub lives: u32,
        pub grid: Grid,
    }

    impl Breakout {
        pub fn new() -> Self {
            Breakout {
                paddle: Paddle::new(SCREEN_WIDTH / 2 - PADDLE_WIDTH / 2, SCREEN_HEIGHT - 60),
                ball: Ball::new(),
                score: 0,
                lives: 10,
                grid: Grid::new(),
            }
        }

        pub fn new_game(&mut self) {
            self.grid.fill_rand();
            self.score = 0;
            self.lives = 10;
        }

        pub fn update(&mut self) {
            self.ball.update(&self.paddle, &mut self.grid, &mut self.score, &mut self.lives);
            if self.lives <= 0 {
                self.new_game();
            }
        }
    }
}

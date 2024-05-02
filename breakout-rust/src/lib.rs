pub mod breakout {

    use rand::Rng;

    pub struct Paddle {
        x: i32,
        y: i32,
        color: sdl2::pixels::Color,
    }

    pub const TILE_W: usize = 1440 / 32;
    pub const TILE_H: usize = 1080 / 2 / 16;

    #[derive(Copy, Clone, Debug)]
    pub struct Block {
        pub color_type: u32,
    }

    pub struct Grid {
        pub blocks: Box<[[Block; TILE_H]; TILE_W]>,
    }

    pub struct Breakout {
        paddle: Paddle,
        pub score: u32,
        pub grid: Grid,
    }

    impl Paddle {
        pub fn new(xpos: i32, ypos: i32) -> Paddle {
            Paddle {
                x: xpos,
                y: ypos,
                color: sdl2::pixels::Color::RGB(255, 255, 255),
            }
        }
    }

    impl Grid {
        fn new() -> Grid {
            let g = Box::new([[Block { color_type: 0 }; TILE_H]; TILE_W]);
            Grid { blocks: g }
        }

        fn fill_rand(&mut self) {
            let mut rng = rand::thread_rng();
            for i in 0..TILE_W {
                for z in 0..TILE_H {
                    self.blocks[i][z].color_type = rng.gen_range(1..8);
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

    impl Breakout {
        pub fn new() -> Self {
            Breakout {
                paddle: Paddle::new(1280 / 2 - (50), 500),
                score: 0,
                grid: Grid::new(),
            }
        }
        pub fn new_game(&mut self) {
            self.grid.fill_rand();
        }
    }
}

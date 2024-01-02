use rand::Rng;
use std::io::Write;
use raylib::prelude::*;
use raylib::core::text::measure_text;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::consts::KeyboardKey;
use raylib::color::Color;

const HEIGHT: usize = 256;
const WIDTH : usize = 256;
use State::{Dead, Alive, Dying, Conductor};
const GOL: [[State; 9]; 2] = [[Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead], 
                              [Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Dead]];

const BB: [[State; 9]; 3] = [[Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead], 
                             [Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying],
                             [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead]];
                           
                           
const SEED: [[State; 9]; 2] = [[Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead], 
                               [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead]];

const DAYNIGHT: [[State; 9]; 2] = [[Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Alive], 
                                   [Dead, Dead, Dead, Alive, Alive, Dead, Alive, Alive, Alive]];


const WIREWORLD: [[State; 9]; 4] = [[Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Alive], 
                                    [Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying],
                                    [Conductor, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor],
                                    [Conductor, Alive, Alive, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor]];

#[derive(Copy, Clone, PartialEq, Debug)]
enum State{
    Dead,
    Alive,
    Dying,
    Conductor,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    GOL,
    SEED,
    BB,
    DAYNIGHT,
    WIREWORLD,
    
}

impl State {
    fn as_usize(&self) -> usize {
        match self {
            State::Dead => 0,
            State::Alive => 1,
            State::Dying => 2,
            State::Conductor => 3,
        }
    }
}

fn fill_random_board(board: &mut Vec<[State; WIDTH]>){
    let mut rng = rand::thread_rng();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let a: u32 = rng.gen();
            if a % 2 == 0{ 
                board[i][j] = State::Alive;
            }

        }
    }
}

fn count_neighbours(board: &Vec<[State; WIDTH]>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let i1 = if i == 0 { 0 } else { i-1 };
    let i2 = if i == HEIGHT-1 { HEIGHT } else { i+2 };
    let j1 = if j == 0 { 0 } else { j-1 };
    let j2 = if j == WIDTH-1 { WIDTH } else { j+2 };
    for x in i1..i2 {
        for y in j1..j2 {
            if x == i && y == j {
                continue;
            }
            count += if board[x][y] == State::Alive { 1 } else { 0 };
        }
    }
    count
}

fn _count_dump(board: &Vec<[State; WIDTH]>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if board[i][j] == State::Alive {
            let count = count_neighbours(&board, i, j);
            println!("({}, {}): {}", i, j, count);

            }
        }
    }
}


fn play(board: &mut Vec<[State; WIDTH]>, mode: Mode) -> Vec<[State; WIDTH]> {
    let mut new_board = vec![[Dead; WIDTH]; HEIGHT]; 
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let count = count_neighbours(&board, i, j);
            let idx = board[i][j].as_usize();
            if mode == Mode::GOL {
                new_board[i][j] = GOL[idx][count];
            } else if mode == Mode::BB {
                new_board[i][j] = BB[idx][count];
            }
            else if mode == Mode::SEED {
                new_board[i][j] = SEED[idx][count];
            }
            else if mode == Mode::DAYNIGHT {
                new_board[i][j] = DAYNIGHT[idx][count];
            }
            else {
                new_board[i][j] = WIREWORLD[idx][count];
            }
        }
    }
    new_board
}

fn _save_frame_as_ppm(board: &Vec<[i32; WIDTH]>, offset: usize){
    let filename = format!("data/frame{}.ppm", offset);
    let mut file = std::fs::File::create(filename).unwrap();
    let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    file.write_all(header.as_bytes()).unwrap();
    for i in 0..HEIGHT {
        for j in 0..WIDTH { let color = if board[i][j] == 1 { "67 118 108" } else { "255 255 255" };
            let line = format!("{}\n", color);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}
fn fill_window(board: &Vec<[State; WIDTH]>, d: &mut RaylibDrawHandle ) {
        let color = Color::new(18, 18, 18, 18);
        d.clear_background(color);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let color = if board[i][j] == Alive { Color::new(22, 255, 0, 255) }
                    else if board[i][j] == Dying { Color::new(15, 98, 146, 255) }
                    else if board[i][j] == Conductor { Color::new(255, 237, 0, 255) }
                    else {color };

                d.draw_rectangle((j*8) as i32, (i*4) as i32, 3, 3, color);
            }
        }
}

fn main() {
    let mut board = vec![[State::Dead; WIDTH]; HEIGHT]; 
    fill_random_board(&mut board);

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Game of Life")
        .build();
    rl.set_target_fps(60);
    let mut isplay = false;
    let mut iswin = true;
    let mut mode = Mode::BB;
    let height = rl.get_screen_height();
    let width = rl.get_screen_width();
    let menu_title = "Game of life";
    let menu_items = vec![
        "GOL",
        "BB",
        "SEED",
        "DAYNIGHT",
        "WIREWORLD"
    ];
    let menu_font_size = 55;
    let title_font_size = 80;
    let menu_padding = 10;
    let mut selected = 0;
    let bg = Color::new(18, 18, 18, 18);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        
        if iswin{
            d.clear_background(bg);
            fill_window(&board, &mut d);
            board = play(&mut board, mode);
            std::thread::sleep(std::time::Duration::from_millis(20));

            d.draw_text(menu_title,
                        (width - measure_text(menu_title, title_font_size))/2,
                        50, title_font_size, Color::WHITE);


            let menu_height =
                (menu_items.len() as f32 * menu_font_size as f32 * 1.5 + (menu_items.len() - 1) as f32 * menu_padding as f32)
                as i32;

            let menu_start_y = height / 2 - menu_height / 2 + 40;

            for (index, item) in menu_items.iter().enumerate(){
                let item_y =
                menu_start_y + (index as f32 * menu_font_size as f32 * 1.5 + index as f32 * menu_padding as f32) as i32;
                let mut color = Color::WHITE;
                if index == selected {
                    color = Color::GOLD;
                }

                d.draw_text(item,
                            (width - measure_text(item, menu_font_size))/2,
                             item_y,
                             menu_font_size,color); 
            }
        }
        

        
        if isplay {
            d.clear_background(bg);
            fill_window(&board, &mut d);
            board = play(&mut board, mode);
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        drop(d);
        match rl.get_key_pressed(){
            Some(key) => {
                match key {
                    KeyboardKey::KEY_UP => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyboardKey::KEY_DOWN => {
                        if selected < menu_items.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyboardKey::KEY_Q => {
                        isplay = false;
                        iswin = true;
                    },
                    KeyboardKey::KEY_SPACE => {
                        isplay = !isplay;
                    },
                    KeyboardKey::KEY_R => {
                        if isplay {
                            board = vec![[State::Dead; WIDTH]; HEIGHT];
                            fill_random_board(&mut board);
                        }
                    },
                    KeyboardKey::KEY_ENTER => {
                        match selected {
                            0 => {
                                board = vec![[State::Dead; WIDTH]; HEIGHT];
                                fill_random_board(&mut board);
                                mode = Mode::GOL;
                                iswin = false;
                                isplay = true;
                            }
                            1 => {
                                board = vec![[State::Dead; WIDTH]; HEIGHT];
                                fill_random_board(&mut board);
                                mode = Mode::BB;
                                iswin = false;
                                isplay = true;
                            },
                            2 => {
                                board = vec![[State::Dead; WIDTH]; HEIGHT];
                                fill_random_board(&mut board);
                                mode = Mode::SEED;
                                iswin = false;
                                isplay = true;
                            },
                            3 => {
                                board = vec![[State::Dead; WIDTH]; HEIGHT];
                                fill_random_board(&mut board);
                                mode = Mode::DAYNIGHT;
                                iswin = false;
                                isplay = true;
                            },
                            4 => {
                                board = vec![[State::Dead; WIDTH]; HEIGHT];
                                fill_random_board(&mut board);
                                mode = Mode::WIREWORLD;
                                iswin = false;
                                isplay = true;
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }

    }
}


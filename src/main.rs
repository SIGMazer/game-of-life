use rand::Rng;
use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::core::audio::RaylibAudio;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::core::text::measure_text;
use raylib::prelude::*;
use std::collections::HashMap;
use std::io::Write;

const HEIGHT: usize = 256;
const WIDTH: usize = 256;
const ALIVE_COLOR: Color = Color::new(22, 255, 0, 255);
const DYING_COLOR: Color = Color::new(15, 98, 146, 255);
const CONDUCTOR_COLOR: Color = Color::new(255, 237, 0, 255);
const DEAD_COLOR: Color = Color::new(18, 18, 18, 18);

use State::{Alive, Conductor, Dead, Dying};
const GOL: [[State; 9]; 2] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead],
    [Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Dead],
];

const BB: [[State; 9]; 3] = [
    [Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
    [
        Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying,
    ],
    [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
];

const SEED: [[State; 9]; 2] = [
    [Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead],
    [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead],
];

const DAYNIGHT: [[State; 9]; 2] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Alive],
    [Dead, Dead, Dead, Alive, Alive, Dead, Alive, Alive, Alive],
];

const WIREWORLD: [[State; 9]; 4] = [
    [Dead, Dead, Dead, Alive, Dead, Dead, Alive, Alive, Alive],
    [
        Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying,
    ],
    [
        Conductor, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor,
        Conductor,
    ],
    [
        Conductor, Alive, Alive, Conductor, Conductor, Conductor, Conductor, Conductor, Conductor,
    ],
];

//rule110 pattern
const RULE110: [(usize, State); 8] = [
    (111, Dead),
    (110, Alive),
    (101, Alive),
    (100, Dead),
    (011, Alive),
    (010, Alive),
    (001, Alive),
    (000, Dead),
];

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
enum State {
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
    Rule110,
}

impl Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::GOL => "GOL".to_string(),
            Mode::SEED => "SEED".to_string(),
            Mode::BB => "BB".to_string(),
            Mode::DAYNIGHT => "DAYNIGHT".to_string(),
            Mode::WIREWORLD => "WIREWORLD".to_string(),
            Mode::Rule110 => "Rule110".to_string(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum GameMode {
    Normal,
    Sandbox,
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

fn fill_random_board(board: &mut Vec<[State; WIDTH]>) {
    let mut rng = rand::thread_rng();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let a: u32 = rng.gen();
            if a % 2 == 0 {
                board[i][j] = State::Alive;
            }
        }
    }
}

fn count_neighbours(board: &Vec<[State; WIDTH]>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let i1 = if i == 0 { 0 } else { i - 1 };
    let i2 = if i == HEIGHT - 1 { HEIGHT } else { i + 2 };
    let j1 = if j == 0 { 0 } else { j - 1 };
    let j2 = if j == WIDTH - 1 { WIDTH } else { j + 2 };
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
            } else if mode == Mode::SEED {
                new_board[i][j] = SEED[idx][count];
            } else if mode == Mode::DAYNIGHT {
                new_board[i][j] = DAYNIGHT[idx][count];
            } else if mode == Mode::WIREWORLD {
                new_board[i][j] = WIREWORLD[idx][count];
            } else if mode == Mode::Rule110 {
                let mut rule = HashMap::new();
                for (key, value) in RULE110.iter() {
                    rule.insert(key, value);
                }
                let mut key = 0;
                if i > 0 {
                    key += board[i - 1][j].as_usize() * 100;
                }
                key += board[i][j].as_usize() * 10;
                if i < HEIGHT - 1 {
                    key += board[i + 1][j].as_usize();
                }
                new_board[i][j] = **rule.get(&key).unwrap();
            }
        }
    }
    new_board
}

fn _save_frame_as_ppm(board: &Vec<[i32; WIDTH]>, offset: usize) {
    let filename = format!("data/frame{}.ppm", offset);
    let mut file = std::fs::File::create(filename).unwrap();
    let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    file.write_all(header.as_bytes()).unwrap();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let color = if board[i][j] == 1 {
                "67 118 108"
            } else {
                "255 255 255"
            };
            let line = format!("{}\n", color);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}
fn fill_window(board: &Vec<[State; WIDTH]>, d: &mut RaylibDrawHandle) {
    let color = Color::new(18, 18, 18, 18);
    d.clear_background(color);
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let color = match board[i][j] {
                State::Alive => ALIVE_COLOR,
                State::Dying => DYING_COLOR,
                State::Conductor => CONDUCTOR_COLOR,
                State::Dead => DEAD_COLOR,
            };

            d.draw_rectangle((j * 8) as i32, (i * 4) as i32, 3, 3, color);
        }
    }

    let grid_color = Color::new(0, 0, 0, 255);
    for i in 0..HEIGHT {
        d.draw_line(
            0,
            (i * 4) as i32,
            WIDTH as i32 * 8,
            (i * 4) as i32,
            grid_color,
        );
    }
    for i in 0..WIDTH {
        d.draw_line(
            (i * 8) as i32,
            0,
            (i * 8) as i32,
            HEIGHT as i32 * 4,
            grid_color,
        );
    }
}

fn sandbox(board: &mut Vec<[State; WIDTH]>, d: &mut RaylibDrawHandle) {
    let color = Color::new(18, 18, 18, 18);
    d.clear_background(color);
    let mouse = d.get_mouse_position();
    let x = mouse.x as usize / 8;
    let y = mouse.y as usize / 4;
    if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        board[y][x] = State::Alive;
    }
    if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
        board[y][x] = State::Dead;
    }
    fill_window(&board, d);
}

fn normalise_board(board: &mut Vec<[State; WIDTH]>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if board[i][j] != Alive {
                board[i][j] = Dead;
            }
        }
    }
}

fn main() {
    let mut board = vec![[State::Dead; WIDTH]; HEIGHT];
    fill_random_board(&mut board);

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Game of Life")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let mut isplay = false;
    let mut iswin = true;
    let mut mode = Mode::BB;
    let menu_title = "Game of life";
    let menu_items = vec![
        "GOL",
        "BB",
        "SEED",
        "DAYNIGHT",
        "WIREWORLD",
        "RULE110",
        "SandBox",
    ];

    let mut modes: HashMap<usize, Mode> = HashMap::new();
    modes.insert(0, Mode::GOL);
    modes.insert(1, Mode::BB);
    modes.insert(2, Mode::SEED);
    modes.insert(3, Mode::DAYNIGHT);
    modes.insert(4, Mode::WIREWORLD);
    modes.insert(5, Mode::Rule110);

    let menu_font_size = 50;
    let title_font_size = 80;
    let menu_padding = 10;
    let mut selected = 0;
    let bg = Color::new(18, 18, 18, 18);
    let mut game_mode = GameMode::Normal;

    let mut rlm = RaylibAudio::init_audio_device();
    let mut sound = raylib::core::audio::Sound::load_sound("music.mp3").unwrap_or_else(|err| {
        panic!("Failed to load sound: {}", err);
    });
    rlm.play_sound(&mut sound);
    rlm.set_master_volume(0.3);

    while !rl.window_should_close() {
        let height = rl.get_screen_height();
        let width = rl.get_screen_width();

        let mut d = rl.begin_drawing(&thread);

        if !rlm.is_sound_playing(&sound) {
            rlm.play_sound(&mut sound);
        }

        if iswin {
            d.clear_background(bg);
            fill_window(&board, &mut d);
            board = play(&mut board, mode);
            std::thread::sleep(std::time::Duration::from_millis(20));

            d.draw_text(
                menu_title,
                (width - measure_text(menu_title, title_font_size)) / 2,
                50,
                title_font_size,
                Color::WHITE,
            );

            let menu_height = (menu_items.len() as f32 * menu_font_size as f32 * 1.5
                + (menu_items.len() - 1) as f32 * menu_padding as f32)
                as i32;

            let menu_start_y = height / 2 - menu_height / 2 + 65;

            for (index, item) in menu_items.iter().enumerate() {
                let item_y = menu_start_y
                    + (index as f32 * menu_font_size as f32 * 1.5
                        + index as f32 * menu_padding as f32) as i32;
                let mut color = Color::WHITE;
                if index == selected {
                    color = Color::GOLD;
                }

                d.draw_text(
                    item,
                    (width - measure_text(item, menu_font_size)) / 2,
                    item_y,
                    menu_font_size,
                    color,
                );
            }
        }

        if game_mode == GameMode::Sandbox {
            d.draw_text(&mode.to_string(), 0, 0, 32, Color::WHITE);
            sandbox(&mut board, &mut d);
        }

        if isplay {
            d.clear_background(bg);
            d.draw_text(&mode.to_string(), 0, 0, 32, Color::WHITE);
            fill_window(&board, &mut d);
            match game_mode {
                GameMode::Normal => {
                    board = play(&mut board, mode);
                }
                GameMode::Sandbox => {
                    board = play(&mut board, mode);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }

        drop(d);

        match rl.get_key_pressed() {
            Some(key) => match key {
                KeyboardKey::KEY_W => {
                    if selected > 0 && iswin {
                        selected -= 1;
                    } else if selected == 0 && iswin {
                        selected = menu_items.len() - 1;
                    }
                }
                KeyboardKey::KEY_S => {
                    if selected < menu_items.len() - 1 && iswin {
                        selected += 1;
                    } else if selected == menu_items.len() - 1 && iswin {
                        selected = 0;
                    }
                }
                KeyboardKey::KEY_Q => {
                    isplay = false;
                    iswin = true;
                }
                KeyboardKey::KEY_SPACE => {
                    if !iswin {
                        isplay = !isplay;
                    }
                }
                KeyboardKey::KEY_K => {
                    if !iswin {
                        if selected < menu_items.len() - 2 {
                            selected += 1;
                        } else if selected >= menu_items.len() - 2 {
                            selected = 0;
                        }
                        mode = *modes.get(&selected).unwrap();
                        normalise_board(&mut board);
                    }
                }
                KeyboardKey::KEY_J => {
                    if !iswin {
                        if selected > 0 {
                            selected -= 1;
                        } else if selected == 0 {
                            selected = menu_items.len() - 2;
                        }
                        mode = *modes.get(&selected).unwrap();
                        normalise_board(&mut board);
                    }
                }
                KeyboardKey::KEY_R => {
                    if isplay {
                        board = vec![[State::Dead; WIDTH]; HEIGHT];
                        fill_random_board(&mut board);
                    }
                }
                KeyboardKey::KEY_D => {
                    if isplay {
                        board = vec![[State::Dead; WIDTH]; HEIGHT];
                    }
                }
                KeyboardKey::KEY_ENTER => {
                    if iswin {
                        if selected == menu_items.len() - 1 {
                            game_mode = GameMode::Sandbox;
                            board = vec![[State::Dead; WIDTH]; HEIGHT];
                            iswin = false;
                        } else {
                            mode = *modes.get(&selected).unwrap();
                            normalise_board(&mut board);
                            game_mode = GameMode::Normal;
                            isplay = true;
                            iswin = false;
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
    }
}

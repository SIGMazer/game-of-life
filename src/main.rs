use rand::Rng;
use ncurses::*;
use std::io::Write;
use raylib::prelude::*;

const HEIGHT: usize = 256;
const WIDTH : usize = 256;
use State::{Dead, Alive, Dying};
const GOL: [[State; 9]; 2] = [[Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead], 
                              [Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Dead]];

//brain's brain
const BB: [[State; 9]; 3] = [[Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead], 
                             [Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying, Dying],
                             [Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead]];
                           
                           

#[derive(Copy, Clone, PartialEq, Debug)]
enum State{
    Dead,
    Alive,
    Dying,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mode {
    GOL,
    BB,
    
}

impl State {
    fn as_usize(&self) -> usize {
        match self {
            State::Dead => 0,
            State::Alive => 1,
            State::Dying => 2,
        }
    }
}

fn _print_board(board: &Vec<[State; WIDTH]>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if board[i][j] == State::Dead {
                addstr("-");
            } if board[i][j] == State::Alive {
                addstr("#");
            } if board[i][j] == State::Dying {
                addstr("X");
            }

        }
        addstr("\n");
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

fn main() {
    let mut board = vec![[State::Dead; WIDTH]; HEIGHT]; 
    fill_random_board(&mut board);

    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Game of Life")
        .build();
    let mut isplay = false;
    let mut mode = Mode::BB;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let color = Color::new(18, 18, 18, 18);
        d.clear_background(color);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let color = if board[i][j] == Alive { Color::new(22, 255, 0, 255) }
                    else if board[i][j] == Dying { Color::new(15, 98, 146, 255) }
                    else {color };

                d.draw_rectangle((j*8) as i32, (i*4) as i32, 3, 3, color);
            }
        }
        if isplay {
            board = play(&mut board, mode);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        if d.is_key_pressed(raylib::consts::KeyboardKey::KEY_Q) {
            break;
        }
        if d.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {
            fill_random_board(&mut board);
        }
        if d.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
            isplay = !isplay; 
        }
        if d.is_key_pressed(raylib::consts::KeyboardKey::KEY_M) {
            mode = match mode {
                Mode::GOL => Mode::BB,
                Mode::BB => Mode::GOL,
            }
        }
    }
}


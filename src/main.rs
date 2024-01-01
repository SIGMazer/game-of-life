use rand::Rng;
use ncurses::*;
use std::io::Write;

const HEIGHT: usize = 480;
const WIDTH : usize = 640;
const RULES: [[i32; 9]; 2] = [[0, 0, 0, 1, 0, 0, 0, 0, 0], 
                              [0, 0, 1, 1, 0, 0, 0, 0, 0]];

fn print_board(board: &Vec<[i32; WIDTH]>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if board[i][j] == 0 {
                addstr("-");
            } else{
                addstr("#");
            }

        }
        addstr("\n");
    }
}

fn fill_random_board(board: &mut Vec<[i32; WIDTH]>){
    let mut rng = rand::thread_rng();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let a: u32 = rng.gen();
            if a % 2 == 0{ 
                board[i][j] = 1;
            }

        }
    }
}

fn count_neighbours(board: &Vec<[i32; WIDTH]>, i: usize, j: usize) -> usize {
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
            count += board[x][y] as usize;
        }
    }
    count
}

fn _count_dump(board: &Vec<[i32; WIDTH]>) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if board[i][j] == 1 {
            let count = count_neighbours(&board, i, j);
            println!("({}, {}): {}", i, j, count);

            }
        }
    }
}


fn play(board: &mut Vec<[i32; WIDTH]>) -> Vec<[i32; WIDTH]> {
    let mut new_board = vec![[0; WIDTH]; HEIGHT]; 
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let count = count_neighbours(&board, i, j);
            new_board[i][j] = RULES[board[i][j] as usize][count];
        }
    }
    new_board
}

fn save_frame_as_ppm(board: &Vec<[i32; WIDTH]>, offset: usize){
    let filename = format!("data/frame{}.ppm", offset);
    let mut file = std::fs::File::create(filename).unwrap();
    let header = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
    file.write_all(header.as_bytes()).unwrap();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let color = if board[i][j] == 1 { "67 118 108" } else { "255 255 255" };
            let line = format!("{}\n", color);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let mut board = vec![[0; WIDTH]; HEIGHT]; 
    fill_random_board(&mut board);

    initscr();

    print_board(&board);
    refresh();

    loop {
        match getch() as u8 as char {
            'q' => break,
            'r' => {
                clear();
                board = play(&mut board);
                print_board(&board);
                refresh();
            },
            'p' => {
                let mut _offset = 0;
                loop {
                    clear();
                    board = play(&mut board); print_board(&board);
                    save_frame_as_ppm(&board, _offset);
                    _offset += 1;
                    refresh();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            },
            _ => {}
        }
    }
    endwin();
}


extern crate rand;
extern crate termion;
use crate::menu::main_menu;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use termion::clear;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
/*
PURPOSE: A struct that holds information regarding the player of the maze runner.
INPUT: none
OUTPUT: none
 */
pub struct Player {
    pub x: u64,
    pub y: u64,
    pub wall_smashes: u64,
    pub underfoot: char,
}
/*
PURPOSE: A struct that holds all information regarding the maze.
INPUT: none
OUTPUT: none
 */
pub struct Maze {
    pub start_x: u64,
    pub start_y: u64,
    pub finish_x: u64,
    pub finish_y: u64,
    pub map: Vec<(Vec<(char)>)>,
}

/*
PURPOSE: When a maze is randomly generated, this function will save that maze to the mazes directory so a player may replay any maze they had generated.
INPUT: A 2D vec of chars and a string that is the name of the maze.
OUTPUT: Bool used in tests to determine if the function was successful.
 */
pub fn save_maze_to_file(maze: &mut std::vec::Vec<(Vec<(char)>)>, name: String) -> bool {
    let p = env::current_dir().unwrap(); //ref 3
    let p2 = p.display().to_string();
    let p3 = p2.split('/');
    let mut path: String = String::new();
    for i in p3 {
        if i == "maze-runner" || i == "maze_runner" {
            path = path + &i.to_string() + "/src/mazes/";
            break;
        }
        path = path + &i.to_string() + "/";
    }
    if File::open(path.clone()).is_ok() {
        //ref 3 start
        let mut data: String;
        let mut f = File::create(path + &name).expect("Could not find mazes directory.");
        for i in 0..maze.len() {
            data = maze[i].clone().into_iter().collect();
            data += &"\n".to_string();
            f.write_all(data.as_bytes()).expect("Could not save maze.");
        }
        return true;
    //ref 3 end
    } else {
        println!("{}", clear::All); //ref 6
        println!("Mazes file not found!");
        main_menu();
    }
    false
}
/*
PURPOSE: This function will generate the randomly generated maze to the player's specifications.
INPUT: a vec with the width and height of the maze, and the name of the maze as a string.
OUTPUT: none
 */
pub fn generate_maze(info: Vec<String>, name: String) {
    let mut maze = Vec::new();
    let mut iter = 0;
    let width: u64 = info[0].parse::<u64>().unwrap();
    let height: u64 = info[1].parse::<u64>().unwrap();
    for s in 0..height {
        maze.push(Vec::new());
        for i in 0..width {
            if s == height - 1 || i == width - 1 || s == 0 || i == 0 {
                maze[iter].push('x');
            } else {
                maze[iter].push('_');
            }
        }
        iter += 1;
    }
    //The following block of code generates random location for the start, end, and wall smashes within the maze.
    let endx: usize = maze.len() as usize - 2;
    let endy: usize = maze[0].len() as usize - 2;
    recursive_maze_creation(&mut maze, 1 as usize, endx, 1 as usize, endy, 0);
    let mut ran_x = rand::thread_rng().gen_range(1, maze.len() - 2);
    let mut ran_y = rand::thread_rng().gen_range(1, maze[0].len() - 2);
    maze[ran_x][ran_y] = 's';
    ran_x = rand::thread_rng().gen_range(1, maze.len() - 2);
    ran_y = rand::thread_rng().gen_range(1, maze[0].len() - 2);
    maze[ran_x][ran_y] = 'f';
    let wall_smashes = rand::thread_rng().gen_range(maze.len() / 10, maze.len() / 4);
    for _i in 0..wall_smashes {
        ran_x = rand::thread_rng().gen_range(1, maze.len() - 2);
        ran_y = rand::thread_rng().gen_range(1, maze[0].len() - 2);
        maze[ran_x][ran_y] = 'v';
    }
    save_maze_to_file(&mut maze, name);
    begin_game(maze);
}

/*
PURPOSE: This function triggers the algorithm that will create the maze.  It does this by making two bisecting lines, opening a hole in three of the four resulting chambers and then repeating the process on each created maze section until the maze is complete.
INPUT: A 2D vec of chars(the maze), the vertical start and end points of a chamber (xstart and xend), the horizontal start and end points of a chamber (ystart and yend), and a usice that lets the loading message know how many dots to add to the end of the loading message.  The loading message is used in the event that maze generation takes a while.
OUTPUT: none
 */
//The idea for this algorithm was found in reference 7.
pub fn recursive_maze_creation(
    mut maze: &mut std::vec::Vec<(Vec<(char)>)>,
    xstart: usize,
    xend: usize,
    ystart: usize,
    yend: usize,
    mut dot: usize,
) {
    if dot >= 100 {
        println!("{}", clear::All); //ref 6
        print!("Loading");
        dot = 0;
    } else {
        if dot == 20 {
            print!(".");
        }
        dot += 1;
    }
    if xend - xstart <= 1 || yend - ystart <= 1 {
        return;
    }
    let vert_wall = rand::thread_rng().gen_range(ystart + 1, yend);
    let hor_wall = rand::thread_rng().gen_range(xstart + 1, xend);
    for i in xstart..=xend {
        maze[i][vert_wall] = 'x';
    }
    for i in ystart..=yend {
        maze[hor_wall][i] = 'x';
    }

    let mut hole_punch = rand::thread_rng().gen_range(1, 5);
    let mut sides_chosen = 0;
    let mut one_chosen = false;
    let mut two_chosen = false;
    let mut three_chosen = false;
    let mut four_chosen = false;

    while sides_chosen < 3 {
        //hole in vert wall before hor wall intersection.
        if hole_punch == 1 && !one_chosen {
            let vert_hole = rand::thread_rng().gen_range(xstart, hor_wall);
            maze[vert_hole][vert_wall] = '_';
            one_chosen = true;
            sides_chosen += 1;
        }
        //hole in hor wall before vert wall intersection.
        else if hole_punch == 2 && !two_chosen {
            let hor_hole = rand::thread_rng().gen_range(ystart, vert_wall);
            maze[hor_wall][hor_hole] = '_';
            two_chosen = true;
            sides_chosen += 1;
        }
        //hole in vert wall after hor wall intersection.
        else if hole_punch == 3 && !three_chosen {
            let vert_hole;
            if hor_wall + 1 == xend {
                vert_hole = hor_wall + 1;
            } else {
                vert_hole = rand::thread_rng().gen_range(hor_wall + 1, xend);
            }
            maze[vert_hole][vert_wall] = '_';
            three_chosen = true;
            sides_chosen += 1;
        }
        //hole in hor wall after vert wall intersection.
        else if hole_punch == 4 && !four_chosen {
            let hor_hole;
            if vert_wall + 1 == yend {
                hor_hole = vert_wall + 1;
            } else {
                hor_hole = rand::thread_rng().gen_range(vert_wall + 1, yend);
            }
            maze[hor_wall][hor_hole] = '_';
            four_chosen = true;
            sides_chosen += 1;
        }
        hole_punch = rand::thread_rng().gen_range(1, 5);
    }
    //Recursively call the same function for each chamber created by the bisecting lines.
    recursive_maze_creation(&mut maze, xstart, hor_wall - 1, ystart, vert_wall - 1, dot);
    recursive_maze_creation(&mut maze, xstart, hor_wall - 1, vert_wall + 1, yend, dot);
    recursive_maze_creation(&mut maze, hor_wall + 1, xend, ystart, vert_wall - 1, dot);
    recursive_maze_creation(&mut maze, hor_wall + 1, xend, vert_wall + 1, yend, dot);
}
/*
PURPOSE: Load in a maze if the player choosese to load in a maze file.
INPUT: The name of the file to load.
OUTPUT: A 2D vec of chars (the maze).
 */
pub fn load_maze(file_name: String) -> Vec<(Vec<(char)>)> {
    let p = env::current_dir().unwrap(); //ref 3
    let p2 = p.display().to_string();
    let p3 = p2.split('/');
    let mut path: String = String::new();
    for i in p3 {
        if i == "maze-runner" || i == "maze_runner" {
            path = path + &i.to_string() + "/src/mazes/" + &file_name;
            break;
        }
        path = path + &i.to_string() + "/";
    }
    //start ref 3
    let mut data = String::new();
    if File::open(path.clone()).is_ok() {
        let mut file = File::open(path).expect("Could not find directory 'mazes'");
        file.read_to_string(&mut data)
            .expect("Unable to read a line.");
        //end ref 3
        let mut maze = Vec::new();
        let mut iter = 0;
        let lines = data.split('\n');
        for s in lines {
            maze.push(Vec::new());
            for i in s.chars() {
                maze[iter].push(i);
            }
            iter += 1;
        }
        //This loop tests if any loaded maze if rectangular and returns an error if not.
        for s in 0..maze.len() - 1 {
            if maze[0].len() != maze[s].len() {
                println!("ERROR: Length of each line is not uniform, please edit maze to have uniform line length.  Returing to main menu.");
                //start ref 8
                let mut stdout = stdout();
                stdout
                    .write(b"Press Enter to retrn to main menu...")
                    .unwrap();
                stdout.flush().unwrap();
                stdin().read(&mut [0]).unwrap();
                //end ref 8
                println!("{}", clear::All); //ref 6
                main_menu();
            }
        }
        return maze;
    } else {
        println!("{}", clear::All); //ref 6
        println!("Could not find a maze called {}", file_name);
        main_menu();
    }
    let empty = Vec::new();
    empty
}
/*
PURPOSE:This function sets up the player into the maze, sets the finish points, and begins the game loop.
INPUT: A 2D vec of chars
OUTPUT: none
 */
pub fn begin_game(maize: Vec<(Vec<(char)>)>) {
    let mut player1 = Player {
        x: 0,
        y: 0,
        wall_smashes: 5 + maize.len() as u64 / 100 + maize[0].len() as u64 / 100,
        underfoot: 's',
    };
    let mut maze = Maze {
        start_x: 0,
        start_y: 0,
        finish_x: 0,
        finish_y: 0,
        map: maize.clone(),
    };
    let points = find_maze_points(&maze.map);
    //This if statement check to see if there is data on the start and end points of the maze.  The default value for the points was maze.map.len()+1 and the if statement checks to see if any of the points have not been changed.  The points are the coordinates of the start and end point of the maze.
    if points[0] == maze.map.len() as u64 + 1
        || points[1] == maze.map.len() as u64 + 1
        || points[2] == maze.map.len() as u64 + 1
        || points[3] == maze.map.len() as u64 + 1
    {
        println!("ERROR: Could not find starting and finish points.  Returing to main menu.");
        //start ref 8
        let mut stdout = stdout();
        stdout
            .write(b"Press Enter to retrn to main menu...")
            .unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
        //end ref 8
        println!("{}", clear::All); //ref 6
        return main_menu();
    }
    player1.x = points[0];
    player1.y = points[1];
    maze.start_x = points[0];
    maze.start_y = points[1];
    maze.finish_x = points[2];
    maze.finish_y = points[3];
    println!("{}", clear::All); //ref 6
    maze.map[player1.x as usize][player1.y as usize] = 'U';
    display_maze(&maze, &player1);
    game_loop(player1, maze);
}
/*
PURPOSE: The main loop of the game, it continues to loop until the player quits or finishes the maze.
INPUT: A player struct and a maze struct.
OUTPUT: none
 */
fn game_loop(mut player1: Player, mut maze: Maze) {
    let mut direction;
    //The game loop will continue until the player overlaps with the finish character.
    while player1.underfoot != 'f' {
        direction = get_input_direction();
        if direction == 'e' {
            print!("{}", color::Fg(color::Reset));
            exit(0);
        }
        //If the player enters a 'q', return player to main menu.
        if direction == 'q' {
            println!("{}", clear::All); //ref 6
            return main_menu();
        }
        //The player decides to go up.  The statment will check to make sure that the walk attempt ends in walkable land.
        if direction == 'u' && player1.x > 0 {
            if maze.map[player1.x as usize - 1][player1.y as usize] == 'x' {
                if player1.wall_smashes > 0 {
                    println!("You smash through the wall.");
                    player1.wall_smashes -= 1;
                    maze.map[player1.x as usize - 1][player1.y as usize] = '_';
                    display_maze(&maze, &player1);
                } else {
                    println!("You smack face first into a wall.");
                }
                continue;
            } else if maze.map[player1.x as usize - 1][player1.y as usize] == 'v' {
                player1.wall_smashes += 1;
                maze.map[player1.x as usize - 1][player1.y as usize] = '_';
                display_maze(&maze, &player1);
            } else {
                maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
                player1.x -= 1;
                player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
                maze.map[player1.x as usize][player1.y as usize] = 'U';
                display_maze(&maze, &player1);
            }
        //The player decides to go left.  The statment will check to make sure that the walk attempt ends in walkable land.
        } else if direction == 'l' && player1.y > 0 {
            if maze.map[player1.x as usize][player1.y as usize - 1] == 'x' {
                if player1.wall_smashes > 0 {
                    println!("You smash through the wall.");
                    player1.wall_smashes -= 1;
                    maze.map[player1.x as usize][player1.y as usize - 1] = '_';
                    display_maze(&maze, &player1);
                } else {
                    println!("You smack face first into a wall.");
                }
                continue;
            } else if maze.map[player1.x as usize][player1.y as usize - 1] == 'v' {
                player1.wall_smashes += 1;
                maze.map[player1.x as usize][player1.y as usize - 1] = '_';
                display_maze(&maze, &player1);
            } else {
                maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
                player1.y -= 1;
                player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
                maze.map[player1.x as usize][player1.y as usize] = 'U';
                display_maze(&maze, &player1);
            }
        //The player decides to go down.  The statment will check to make sure that the walk attempt ends in walkable land.
        } else if direction == 'd' && player1.x + 1 < maze.map.len() as u64 - 1 {
            if maze.map[player1.x as usize + 1][player1.y as usize] == 'x' {
                if player1.wall_smashes > 0 {
                    println!("You smash through the wall.");
                    player1.wall_smashes -= 1;
                    maze.map[player1.x as usize + 1][player1.y as usize] = '_';
                    display_maze(&maze, &player1);
                } else {
                    println!("You smack face first into a wall.");
                }
                continue;
            } else if maze.map[player1.x as usize + 1][player1.y as usize] == 'v' {
                player1.wall_smashes += 1;
                maze.map[player1.x as usize + 1][player1.y as usize] = '_';
                display_maze(&maze, &player1);
            } else {
                maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
                player1.x += 1;
                player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
                maze.map[player1.x as usize][player1.y as usize] = 'U';
                display_maze(&maze, &player1);
            }
        //The player decides to go right.  The statment will check to make sure that the walk attempt ends in walkable land.
        } else if direction == 'r' && player1.y + 1 < maze.map[player1.x as usize].len() as u64 {
            if maze.map[player1.x as usize][player1.y as usize + 1] == 'x' {
                if player1.wall_smashes > 0 {
                    println!("You smash through the wall.");
                    player1.wall_smashes -= 1;
                    maze.map[player1.x as usize][player1.y as usize + 1] = '_';
                    display_maze(&maze, &player1);
                } else {
                    println!("You smack face first into a wall.");
                }
                continue;
            } else if maze.map[player1.x as usize][player1.y as usize + 1] == 'v' {
                player1.wall_smashes += 1;
                maze.map[player1.x as usize][player1.y as usize + 1] = '_';
                display_maze(&maze, &player1);
            } else {
                maze.map[player1.x as usize][player1.y as usize] = player1.underfoot;
                player1.y += 1;
                player1.underfoot = maze.map[player1.x as usize][player1.y as usize];
                maze.map[player1.x as usize][player1.y as usize] = 'U';
                display_maze(&maze, &player1);
            }
        } else {
            println!("You cannot go that way!");
        }
    }
    println!("{}", clear::All); //ref 6
    println!(
        "{}CONGRATS! YOU WON AND EXITED THE MAZE!{}",
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );
    main_menu();
}

/*
PURPOSE: Reads player input and then returns a char code to tell the game loop what kind of input it is dealing with.
INPUT: none
OUTPUT: a single char.
 */
//beign reference 6
fn get_input_direction() -> char {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => return 'u',
            Key::Right => return 'r',
            Key::Left => return 'l',
            Key::Down => return 'd',
            Key::Char('w') => return 'u',
            Key::Char('a') => return 'l',
            Key::Char('s') => return 'd',
            Key::Char('d') => return 'r',
            Key::Ctrl(_c) => return 'e',
            Key::Char('q') => return 'q',
            _ => return 'x',
        }
    }
    stdout.flush().unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    'x'
}
//end reference 6

/*
PURPOSE: After each move the player makes, the commandl line is cleared and an updated maze is displayed.
INPUT: a maze struct and a player struct.
OUTPUT: a bool for testing purposes.
 */
pub fn display_maze(maze: &Maze, player1: &Player) -> bool {
    println!("{}", clear::All); //ref 6
    let vert_window: u64 = 10;
    let hor_window: u64 = 20;
    let mut start_i: i32 = player1.x as i32 - vert_window as i32;
    let mut end_i: i32 = player1.x as i32 + vert_window as i32;
    let mut start_j: i32 = player1.y as i32 - hor_window as i32;
    let mut end_j: i32 = player1.y as i32 + hor_window as i32;
    if start_i < 0 {
        start_i = 0;
    }
    if end_i >= maze.map.len() as i32 {
        end_i = maze.map.len() as i32 - 1;
    }
    if start_j < 0 {
        start_j = 0;
    }
    if end_j >= maze.map[0].len() as i32 {
        end_j = maze.map[0].len() as i32 - 1;
    }
    println!("Location:{},{} ", player1.x, player1.y);
    println!("Exit: {},{}", maze.finish_x, maze.finish_y);
    println!("Wall smashes: {}", player1.wall_smashes);
    println!("Pres [q] to quit back to main menu.");
    for i in start_i..end_i {
        for j in start_j..=end_j {
            if maze.map[i as usize][j as usize] == '_' {
                print!(
                    "{}{}",
                    color::Fg(color::Green),
                    maze.map[i as usize][j as usize].to_string()
                );
            } else if maze.map[i as usize][j as usize] == 'U' {
                print!(
                    "{}{}",
                    color::Fg(color::Cyan),
                    maze.map[i as usize][j as usize].to_string()
                );
            } else if maze.map[i as usize][j as usize] == 'f'
                || maze.map[i as usize][j as usize] == 's'
            {
                print!(
                    "{}{}",
                    color::Fg(color::LightYellow),
                    maze.map[i as usize][j as usize].to_string()
                );
            } else if maze.map[i as usize][j as usize] == 'v' {
                print!(
                    "{}{}",
                    color::Fg(color::Magenta),
                    maze.map[i as usize][j as usize].to_string()
                );
            } else {
                print!(
                    "{}{}",
                    color::Fg(color::Reset),
                    maze.map[i as usize][j as usize].to_string()
                );
            }
        }
        println!();
    }
    print!("{}", color::Fg(color::Reset));
    true
}

/*
PURPOSE: Finds the start and finish points of the maze to populate the player and maze struct at the beginning of the game.
INPUT: a 2d Vec of chars (the maze).
OUTPUT: a 4-element array that contains the coordinates for the start and finish point of a maze.
 */
pub fn find_maze_points(maze: &[Vec<(char)>]) -> [u64; 4] {
    let mut coordinates: [u64; 4] = [
        (maze.len() + 1) as u64,
        (maze.len() + 1) as u64,
        (maze.len() + 1) as u64,
        (maze.len() + 1) as u64,
    ];
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == 's' {
                coordinates[0] = i as u64;
                coordinates[1] = j as u64;
            } else if maze[i][j] == 'f' {
                coordinates[2] = i as u64;
                coordinates[3] = j as u64;
            }
            if coordinates[0] != (maze.len() + 1) as u64
                && coordinates[1] != (maze.len() + 1) as u64
                && coordinates[2] != (maze.len() + 1) as u64
                && coordinates[3] != (maze.len() + 1) as u64
            {
                return coordinates;
            }
        }
    }
    coordinates
}

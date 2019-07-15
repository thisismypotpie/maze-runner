
mod menu;
mod running_alg;

fn main() {

  print!("{}[2J", 27 as char);//Ref. 1
  println!("DISCLAIMER!!!!!!!!: Throughout the code there will be references to code that I used throughout the project that was not my own however they will not be listed in a traditional way.  Instead if you see a comment that has a 'Ref. #' this is a reference to a link in my references.txt page in my primary directory for tis project.  This will allow for cleaner and more readable code.  If you would like to see each reference link please go the to references.txt page.  Now back to the game. \n\n");
  let _maze_selection:i32 =  menu::main_menu(); 
  if _maze_selection == 2 {//Load in maze.	
    menu:: load_in_maze();
  }
  else if _maze_selection > 9 {//Generate a maze.
    running_alg:: generate_maze();
  }
  //let _traversal_selection:i32 = menu:: maze_solving_strategy();
}
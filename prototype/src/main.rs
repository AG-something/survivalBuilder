use std::io;

pub mod grid;

fn main() {
    let mut dim1_input = String::new();
    println!("How many lines do you want?");
    io::stdin().read_line(&mut dim1_input).expect("Failed to read line");
    let dim1 : u64 = dim1_input.trim().parse().expect("Invalid dimensions");

    let mut dim2_input = String::new();
    println!("How many columns do you want?");
    io::stdin().read_line(&mut dim2_input).expect("Failed to read line");
    let dim2 : u64 = dim2_input.trim().parse().expect("Invalid dimensions");

    
    let g = crate::grid::grid::grid::Grid::new(dim1, dim2);
    loop{
	let mut user_input = String::new();
	
	g.print_grid();
	println!("Select action :");
	println!("\t(1)  Inspect a tile");
	println!("\t(2)  Exit");
	io::stdin().read_line(&mut user_input).expect("Failed to read line");

	let user_choice : u64 = user_input.trim().parse().expect("Please type a number");
	match user_choice {
	    | 1 => {
		println!("Enter x coordinate: ");
		let mut x_input = String::new();
		io::stdin().read_line(&mut x_input).expect("Failed to read line");
		let x_coordinate : u64 = x_input.trim().parse().expect("Please type a number");

		println!("Enter y coordinate: ");
		let mut y_input = String::new();
		io::stdin().read_line(&mut y_input).expect("Failed to read line");
		let y_coordinate : u64 = y_input.trim().parse().expect("Please type a number");
		if (0 <= x_coordinate) && (0 <= y_coordinate) && (x_coordinate < dim1) && (y_coordinate < dim2) {
		    g.inspect_tile(x_coordinate as usize, y_coordinate as usize);
		} else {
		    println!("Those coordinates are off the board!");
		}
	    }
	    | 2 => {
		println!("Exiting the game");
		break;
	    }
	    | _ => {
		println!("This option is not currently supported");
	    }
	}
	
    }
}

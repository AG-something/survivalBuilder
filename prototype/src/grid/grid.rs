pub mod grid{
    use crate::grid::tile::tile;
    use crate::grid::resource::resource;
    use rand::Rng;

    #[derive(Debug)]
    pub struct Grid {
	tiles : Vec<Vec<tile::Tile>>,
    }

    impl Grid {
	pub fn new(dim1 : u64, dim2 : u64) -> Grid{
	    Grid{
		tiles : {
		    let mut ts = Vec::new();
		    let mut rng = rand::thread_rng();
		    let mut i : u64 = 0;
		    while i < dim1 {
			let mut ts_i = Vec::new();
			let mut j : u64 = 0;
			while j < dim2 {
			    let resource_dice : u64 = rng.gen_range(0..6);
			    let res : resource::Resource = {
				let quantity : u64 = rng.gen_range(10..100);
				match resource_dice {
				    | 0 => resource::Resource::new(quantity, resource::ResourceKind::Water),
				    | 1 => resource::Resource::new(quantity, resource::ResourceKind::Iron),
				    | 2 => resource::Resource::new(quantity, resource::ResourceKind::Carbon),
				    | _ => resource::Resource::new(0, resource::ResourceKind::None),
				}
			    };
			    let terrain_dice : u64 = rng.gen_range(0..4);
			    let ter = {
				match terrain_dice {
				    | 0 => tile::Terrain::Mountains,
				    | 1 => tile::Terrain::Rivers,
				    | 2 => tile::Terrain::Desert,
				    | _ => tile::Terrain::Meadows,
				}
			    };
			    let size_dice : u64 = rng.gen_range(1..3);
			    let t = tile::Tile::new((i,j), ter, res, size_dice);
			    ts_i.push(t);
			    j = j + 1;
			}
			ts.push(ts_i);
			i = i + 1;
		    }
		    ts
		}
	    }
	}

	// Method that pretty prints the grid
	pub fn print_grid(&self){
	    let mut i : usize = 0;
	    let dim1 = self.tiles.len();
	    let dim2 = self.tiles[0].len();

	    // Print the first line with numbers indexing columns
	    print!("   ");
	    while i < dim2 {
		print!("| {:?} ", i);
		i = i + 1;
	    }
	    println!("|");

	    i = 0;
	    let mut c : u8 = 65;
	    while i < dim1 {
		let mut j : usize = 0;
		print!("---");
		while j < dim2 {
		    print!("|---");
		    j = j + 1;
		}
		println!("|");
		
		print!("{:?}", c as char);
		c = c + 1;
		j = 0;
		while j < dim2 {
		    print!("|   ");
		    j = j + 1;
		}
		println!("|");
		i = i + 1;
	    }

	    print!("---");
	    i = 0;
	    while i < dim2 {
		print!("|---");
		i = i + 1;
	    }
	    println!("|");
	}

	// A method that targets a grid location and prints the content of the tile at that location
	pub fn inspect_tile(&self, x : usize, y : usize){
	    let _ = &self.tiles[x][y].print_tile();
	}
    }

}

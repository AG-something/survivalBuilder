mod grid{
    use crate::grid::tile::tile;
    use crate::grid::resource::resource;
    use rand::Rng;
    
    struct Grid {
	tiles : Vec<Vec<tile::Tile>>,
    }

    impl Grid {
	fn new(dim : u64) -> Grid{
	    Grid{
		tiles : {
		    //		    let ts = Vec<Vec<tile::Tile>>.new();
		    let mut ts = Vec::new();
		    let mut rng = rand::thread_rng();
		    let mut i : u64 = 0;
		    while i < dim {
			//			let ts_i = Vec<tile::Tile>>.new();
			let mut ts_i = Vec::new();
			let mut j : u64 = 0;
			while j < dim {
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
    }
}

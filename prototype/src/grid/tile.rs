pub mod tile{
    use crate::grid::resource::resource::Resource;
    use crate::grid::building::building::BuildingSlot;

    #[derive(Debug)]
    enum Owner{
	Player(i64),
	Company(i64),
	None,
    }

    #[derive(Debug)]
    pub enum Terrain{
	Mountains,
	Rivers,
	Desert,
	Meadows,   // Default, no buff no debuff
    }

    #[derive(Debug)]
    pub struct Tile{
	position : (u64, u64),
	owner : Owner,
	terrain : Terrain,
	resources : Resource,
	buildings : Vec<BuildingSlot>,
    }

    impl Tile{
	// Constructor for Tile
	pub fn new(pos : (u64, u64), ter : Terrain, res : Resource, size : u64) -> Tile{
	    Tile{
		position : pos,
		owner : Owner::None,
		terrain : ter,
		resources : res,
		buildings : {
		    let mut b = Vec::new();
		    let mut i = 0;
		    while i < size {
			b.push(BuildingSlot::new());
			i = i + 1;
		    }
		    b
		}
	    }
	}

	// Pretty printing a tile
	pub fn print_tile(&self) {
	    println!("{:?}", self);
	}
    }
}

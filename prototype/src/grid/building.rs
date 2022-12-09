pub mod building{
    use crate::grid::resource::resource::ResourceKind;
    
    #[derive(Debug)]
    pub enum BuildingKind{
	Extractor(ResourceKind, u64),
	House(u64, u64),
	Storage(u64, Vec<(ResourceKind, u64)>),
	None,
    }

    #[derive(Debug)]
    pub struct BuildingSlot{
	building : BuildingKind,
	status : i64,
    }

    impl BuildingSlot{
	fn new(number_of_slots : u64) -> BuildingSlot{
	    BuildingSlot{
		building : BuildingKind::None,
		status : 0,		
	    }
	}

	fn build(&mut self, building_kind : BuildingKind){
	    match self.building {
		| BuildingKind::None => {
		    self.building = building_kind;
		    self.status = 100;
		}
		| _ => {
		    println!("There is already a building here");
		},
	    }
	}

	fn destroy(&mut self){
	    self.building = BuildingKind::None;
	    self.status = 0;
	}
    }
}

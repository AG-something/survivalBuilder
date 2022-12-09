mod tile{
    use crate::grid::resource::resource::Resource;
    use crate::grid::building::building::BuildingSlot;
    
    enum Owner{
	Player(i64),
	Company(i64),
	None,
    }

    enum Terrain{
	Moutains,
	Rivers,
	Desert,
    }

    struct tile{
	owner : Owner,
	terrain : Terrain,
	resources : Vec<Resource>,
	buildings : Vec<BuildingSlot>,
    }

}

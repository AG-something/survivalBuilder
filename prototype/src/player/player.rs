pub mod player{
    use crate::grid::resource::resource::Resource;
    use crate::grid::building::building::BuildingSlot;

    #[derive(Debug, Clone, Copy)]

    //Structure of a single Player playing the game. 
    pub struct Player{
        id : u64,
        civ : str,
        totalpoint : u64,
        workers : Vec<WorkerGroup>,
        Possible_actions : Vec<Action>,
    }



    impl Player{
        pub fn new(id : u64, civ : str, starting_point : u64, Worker_group : Vec<Worker_group>) -> Player {
            Player{
                id : id,
                civ : civ, 
                totalpoint : starting_point,
                worker_group : worker_group,
            }
        }
    }
}
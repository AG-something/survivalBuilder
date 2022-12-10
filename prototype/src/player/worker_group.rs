pub mod worker_group{
    use crate::grid::resource::resouce::Resource;
    use crate::grid::building::building::BuildingSlot;

    #[derive(Debug, Clone, Copy)]
    pub struct Worker_group{
        role : Vec<struct>,
        locaton :tup(u64, u64),
        action_point : u64,
        health_point : u64
    }
    #[derive(Debug, Clone, Copy)]
    pub struct Role{
        active_role(),
        list_of_role_group(list enum)
    }
    impl Worker_group{
        pub fn new(q : Vec<struct>, k : tup(u64, u64, ap : u64, hp : u64) -> Worker_group{
            Worker_group{
                role : q,
                location : k , //implementer rand starting position 
                action_point : ap,
                health_point : hp,
            }
        }
    }
}
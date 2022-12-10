pub mod actions{
    use crate::grid::resource::resource::Resource;
    use crate::grid::building::building::BuildingSlot;
    
    
    pub struct Action{
        conditions : Vec<enum>,
        output : fn, 
        choices : Vec<str>  // All the possible choices of a specific actions
    }
    
    impl Action{
        pub fn new(conditions : Vec<enum>, outout : fn, choices : Vec<enum>){
            Action{
                conditions : conditions, 
                output : output, 
                choices : choice,
            }
        }
    }

    

}
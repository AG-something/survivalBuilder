pub mod resource{
    #[derive(Debug, Clone, Copy)]
    pub enum ResourceKind{
	Water,
	Iron,
	Carbon,
	None,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Resource{
	amount : u64,
	kind : ResourceKind,
    }

    impl Resource{
	pub fn new(q : u64, k : ResourceKind) -> Resource {
	   Resource{
		amount : q,
		kind : k,
	    }
	}
	
	pub fn exploit(&mut self, quantity : u64) -> (u64, ResourceKind) {
	    if self.amount <= quantity {
		self.amount = 0;
		(self.amount, self.kind)
	    } else {
		self.amount = self.amount - quantity;
		let k = self.kind.clone();
		(quantity, k)
	    }   
	}
    }
}

use std::
{
    sync::
    {
        RwLock,
        Arc,
    },
};

#[derive(Clone)]
pub struct CollDataPtr
{
    ptr: Arc<RwLock<super::holder::CollDataHolder>>,
}

impl CollDataPtr
{
    pub fn new() -> CollDataPtr
    {
        CollDataPtr
        {
            ptr: Arc::from(RwLock::from(super::holder::CollDataHolder::new())),
        }
    }

    pub fn from(other: &CollDataPtr) -> CollDataPtr
    {
        CollDataPtr
        {
            ptr: other.ptr.clone(),
        }
    }

    pub fn check_collisions(&self, from: (i32, i32), to: (i32, i32)) -> Vec<(i32, i32)>
    {
        self.ptr.read().expect("could not read collider").check_collisions(from, to)
    }

    pub fn set_world(&self, world: Vec<(i32, i32)>)
    {self.ptr.write().expect("could not write to collider (world)").world = world;}
    pub fn set_entities(&self, entities: Vec<(i32, i32)>)
    {self.ptr.write().expect("could not write to collider (entities)").entities = entities;}
    pub fn set_player(&self, player: (i32, i32))
    {self.ptr.write().expect("could not write to collider (player)").player = player;}
}


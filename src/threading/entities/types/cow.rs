use super::super::
{
    super::
    {
        collision_handler::
        {
            movement::Moveable,
            ptr::CollDataPtr,
        },
    },

    entity::
    {
        Entity,
        EntityType,
    },
};

pub struct Cow
{
    loc: (i32, i32),

    collider: CollDataPtr,
}

impl Cow
{
    pub fn new(ptr: &CollDataPtr) -> Cow
    {
        Cow
        {
            loc: (12, 8),
            collider: ptr.clone(),
        }
    }
}

impl Moveable for Cow
{
    fn get_loc(&self) -> &(i32, i32)
    {&self.loc}

    fn get_loc_mut(&mut self) -> &mut (i32, i32)
    {&mut self.loc}

    fn get_collision_data_ptr(&self) -> CollDataPtr
    {self.collider.clone()}
}

impl Entity for Cow
{
    fn update(&mut self)
    {}

    fn get_type(&self) -> EntityType
    {EntityType::Cow}
}
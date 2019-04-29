extern crate rand;

use super::super::
{
    super::
    {
        collision_handler::
        {
            movement::
            {
                Moveable,
                Direction,
            },
            ptr::CollDataPtr,
        },
    },

    entity::
    {
        Entity,
        EntityType,
    },
};

use rand::
{
    RngCore,
    prelude::SmallRng,
    FromEntropy,
};

pub struct Cow
{
    loc: (i32, i32),

    collider: CollDataPtr,
    rng: SmallRng,
}

impl Cow
{
    pub fn new(ptr: &CollDataPtr) -> Cow
    {
        Cow
        {
            loc: (12, 8),

            collider: ptr.clone(),
            rng: SmallRng::from_entropy(),
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
    {
        let rand = self.rng.next_u32() % 4;

        let dir = match rand
        {
            0 => Direction::Up(1),
            1 => Direction::Down(1),
            2 => Direction::Left(1),
            3 => Direction::Right(1),

            _ => panic!("rng value generated beyond expected"),
        };

        self.move_direction(dir);
    }

    fn get_type(&self) -> EntityType
    {EntityType::Cow}
}
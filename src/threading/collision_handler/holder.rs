use std::
{
    cmp::
    {
        min,
        max,
    }
};

pub struct CollDataHolder
{
    pub entities: Vec<(i32, i32)>,
    pub world: Vec<(i32, i32)>,
    pub player: (i32, i32),
}

impl CollDataHolder
{
    pub fn new() -> CollDataHolder
    {
        CollDataHolder
        {
            entities: vec!(),
            world: vec!(),
            player: (0, 0),
        }
    }

    fn check_collision(&self, point: (i32, i32)) -> bool
    {
        for ent in &self.entities
        {if point == *ent {return true;}}

        for elem in &self.world
        {if point == *elem {return true;}}

        if point == self.player {return true;}

        false
    }

    pub fn check_collisions(&self, from: (i32, i32), to: (i32, i32)) -> Vec<(i32, i32)>
    {
        let mut collisions = vec!();

        for i in 
            min(from.0, to.0)
            ..=
            max(from.0, to.0)
        {
            for j in
                min(from.1, to.1)
                ..=
                max(from.1, to.1)
            {
                if self.check_collision((i, j))
                {collisions.push((i, j));}
            }
        }

        collisions
    }
}
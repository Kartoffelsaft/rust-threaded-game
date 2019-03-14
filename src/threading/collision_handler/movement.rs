pub enum Direction
{
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
}

pub trait Move
{
    fn get_loc(&self) -> &(i32, i32);
    fn get_loc_mut(&mut self) -> &mut (i32, i32);

    fn get_collision_data_ptr(&self) -> super::ptr::CollDataPtr;

    fn move_direction(&mut self, dir: Direction)
    {
        let mut move_to = self.get_loc().clone();

        match dir
        {
            Direction::Up(d) => move_to.1 -= d as i32,
            Direction::Down(d) => move_to.1 += d as i32,
            Direction::Left(d) => move_to.0 -= d as i32,
            Direction::Right(d) => move_to.0 += d as i32,
        }

        let collisions = self.get_collision_data_ptr()
            .check_collisions(self.get_loc().clone(), move_to);

        let mut closest_collision = match dir 
        {
            Direction::Up(d) => d,
            Direction::Down(d) => d,
            Direction::Left(d) => d,
            Direction::Right(d) => d,
        } as i32;

        if collisions.len() > 1
        {
            for collision in collisions
            {
                if collision != *self.get_loc()
                {
                    let dx = (collision.0 - self.get_loc().0).abs();
                    let dy = (collision.1 - self.get_loc().1).abs();
                    let collision_distance = dx + dy;
                    if collision_distance < closest_collision
                    {closest_collision = collision_distance;}
                }
            }
            closest_collision -= 1;
        }

        let mut loc = self.get_loc_mut();
        match dir
        {
            Direction::Up(_) => loc.1 -= closest_collision,
            Direction::Down(_) => loc.1 += closest_collision,
            Direction::Left(_) => loc.0 -= closest_collision,
            Direction::Right(_) => loc.0 += closest_collision,
        }
    }
}

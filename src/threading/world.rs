use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use super::general::ThreadMessage;
use rand::prelude::
{
    thread_rng,
    Rng,
};

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>)
{
    let mut world = WorldData::new(commands, teller);

    loop
    {
        world.parse_commands();

        world.teller.send
            (ThreadMessage::Printer
                (super::printer::PrintCommand::WorldUpdate
                    (world.elements.clone()
        ))).expect("world could not send world data");
    }
}

pub enum WorldCommand
{
    GenerateBuilding(((i32, i32), (u16, u16))),
    CheckCollision((i32, i32), (i32, i32), WorldCollider)
}

pub enum WorldCollider
{
    Player,
}

struct WorldData
{
    commands: Receiver<ThreadMessage>,
    teller: Sender<ThreadMessage>,

    elements: HashMap<(i32, i32), WorldElement>,
}

#[derive(Clone)]
pub enum WorldElement
{
    Wall,
    Floor,
    Door,
}

impl WorldData
{
    pub fn new(c: Receiver<ThreadMessage>, t: Sender<ThreadMessage>) -> WorldData
    {
        WorldData
        {
            commands: c,
            teller: t,

            elements: HashMap::new(),
        }
    }

    fn parse_commands(&mut self)
    {
        match self.commands.recv().expect("world cannot recieve message")
        {
            ThreadMessage::World(c) =>
            {
                match c 
                {
                    WorldCommand::GenerateBuilding(b) => self.place_building(b.0, b.1),
                    WorldCommand::CheckCollision(f, t, c) => self.route_collision_info(f, t, c),
                }
            }

            _ => panic!("world could not enterperet message"),
        }
    }

    pub fn place_building(&mut self, loc: (i32, i32), size: (u16, u16))
    {
        for i in 0..(size.0 + 1) { for j in 0..(size.1 + 1)
        {
            if i == 0 ||
               i == size.0 ||
               j == 0 ||
               j == size.1
            {
                self.place_world_element((loc.0 + i as i32, loc.1 + j as i32), WorldElement::Wall);
            }
            else
            {
                self.place_world_element((loc.0 + i as i32, loc.1 + j as i32), WorldElement::Floor);
            }
        }}

        let mut rng = thread_rng();

        let door_side = rng.gen::<u8>() % 4;
        match door_side
        {
            0 => //top
            {
                let door_shift = rng.gen::<u16>() % (size.0 - 1) + 1;
                self.place_world_element((loc.0 + door_shift as i32, loc.1), WorldElement::Door);
            },
            1 => //left
            {
                let door_shift = rng.gen::<u16>() % (size.1 - 1) + 1;
                self.place_world_element((loc.0, loc.1 + door_shift as i32), WorldElement::Door);
            },
            2 => //bottom
            {
                let door_shift = rng.gen::<u16>() % (size.0 - 1) + 1;
                self.place_world_element((loc.0 + door_shift as i32, loc.1 + size.1 as i32), WorldElement::Door)
            },
            3 => //right
            {
                let door_shift = rng.gen::<u16>() % (size.1 - 1) + 1;
                self.place_world_element((loc.0 + size.0 as i32, loc.1 + door_shift as i32), WorldElement::Door)
            },

            _ => assert!(false),
        }
    }

    fn place_world_element(&mut self, loc: (i32, i32), element: WorldElement)
    {
        self.elements.insert(loc, element);
    }

    fn route_collision_info(&mut self, from: (i32, i32), to: (i32, i32), collider: WorldCollider)
    {
        let coll = self.check_collision(from, to);
        match collider
        {
            WorldCollider::Player => 
            self.teller.send
                (ThreadMessage::Player
                    (super::player::PlayerCommand::Collisions(coll)
            )).expect("world could not send collisions"),
        };
    }

    fn check_collision(&self, from: (i32, i32), to: (i32, i32)) -> Vec<(i32, i32)>
    {
        let mut collisions: Vec<(i32, i32)> = vec!();
        
        let mut top_left: (i32, i32) = (std::i32::MIN, std::i32::MIN);
        let mut bottom_right: (i32, i32) = (std::i32::MIN, std::i32::MIN);

        if from.0 > to.0
        {
            bottom_right.0 = from.0;
            top_left.0 = to.0;
        }
        else
        {
            bottom_right.0 = to.0;
            top_left.0 = from.0;
        }

        if from.1 > to.1
        {
            bottom_right.1 = from.1;
            top_left.1 = to.1;
        }
        else
        {
            bottom_right.1 = to.1;
            top_left.1 = from.1;
        }

        for i in top_left.0..(bottom_right.0 + 1)
        {
            for j in top_left.1..(bottom_right.1 + 1)
            {
                match self.elements.get(&(i, j))
                {
                    Some(e) => match e
                    {
                        WorldElement::Wall => collisions.push((i, j)),

                        _ => ()
                    },

                    None => ()
                }
            }
        }

        collisions
    }
}
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use super::general::ThreadMessage;

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>)
{
    let mut world = WorldData::new();

    loop
    {
        match commands.recv().expect("world cannot recieve message")
        {
            ThreadMessage::World(c) =>
            {
                match c 
                {
                    WorldCommand::GenerateBuilding(b) => world.place_building(b.0, b.1),
                    WorldCommand::CheckCollision(f, t, c) => 
                    {
                        let coll = world.check_collision(f, t);
                        match c
                        {
                            WorldCollider::Player => 
                            teller.send(ThreadMessage::Player(super::player::PlayerCommand::Collisions(coll))),
                        };
                    },
                }
            }

            _ => panic!("world could not enterperet message"),
        }

        teller.send
        (
            ThreadMessage::Printer
            (
                super::printer::PrintCommand::WorldUpdate
                (
                    world.elements.clone()
                )
            )
        ).expect("world could not send world data");
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
    elements: HashMap<(i32, i32), WorldElement>,
}

#[derive(Clone)]
pub enum WorldElement
{
    Wall,
    Floor,
}

impl WorldData
{
    pub fn new() -> WorldData
    {
        WorldData
        {
            elements: HashMap::new(),
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
    }

    fn place_world_element(&mut self, loc: (i32, i32), element: WorldElement)
    {
        self.elements.insert(loc, element);
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
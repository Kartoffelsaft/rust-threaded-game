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
}
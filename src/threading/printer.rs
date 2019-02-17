extern crate term_size;

use std::
{
    sync::mpsc::
    {
        Receiver,
    }, 
    collections::HashMap
};
use super::
{
    general::{ThreadMessage}, 
    world::WorldElement,
    entities::entity::EntityType,
};

const APPEARANCE_PLAYER: char = '@';

const APPEARANCE_WALL: char = '#';
const APPEARANCE_FLOOR: char = '.';
const APPEARANCE_DOOR: char = ']';

const APPEARANCE_ENTITY_COW: char = 'C';

const APPEARANCE_UI_BORDER_BOTTOM: char = '_';

pub fn routine(print_commands: Receiver<ThreadMessage>)
{
    let mut screen = Screen
    {
        commands: print_commands,

        data: vec!(),
        objects: ScreenObjects::new(),
        size: (0usize, 0usize)
    };

    loop
    {
        screen.update_screen_size();

        screen.parse_commands();        

        screen.update_screen();

        let output: String = screen.data.iter().collect();
        print!("{}\n", output);
    }
}

pub enum PrintCommand
{
    Refresh,
    PlayerUpdate((i32, i32)),
    WorldUpdate(HashMap<(i32, i32), WorldElement>),
    EntitiesUpdate(Vec<(EntityType, (i32, i32))>),
    MessageUpdate(String),
}

struct Screen
{
    commands: Receiver<ThreadMessage>,

    data: Vec<char>,
    objects: ScreenObjects,
    size: (usize, usize)
}

struct ScreenObjects
{
    player: (i32, i32),
    world: HashMap<(i32, i32), WorldElement>,
    entities: Vec<(EntityType, (i32, i32))>,
    message_for_player: String
}

impl ScreenObjects
{
    pub fn new() -> ScreenObjects
    {
        ScreenObjects
        {
            player: (0, 0),
            world: HashMap::new(),
            entities: Vec::new(),
            message_for_player: String::new(),
        }
    }
}

impl Screen
{
    fn parse_commands(&mut self)
    {
        match self.commands.recv().unwrap()
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Refresh => self.objects.message_for_player = String::new(),
                PrintCommand::PlayerUpdate(l) => self.objects.player = l,
                PrintCommand::WorldUpdate(w) => self.objects.world = w,
                PrintCommand::EntitiesUpdate(e) => self.objects.entities = e,
                PrintCommand::MessageUpdate(m) => self.objects.message_for_player = m,
            }

            _ => panic!("Printer given unrecognizable command")
        }
    }

    fn place_char
    (
        &mut self,
        character: char, 
        loc: &(usize, usize),
    )
    {
        if loc.0 < self.size.0 &&
        loc.1 < self.size.1
        {
            if let Some(screen_loc) = self.data.get_mut(loc.1*self.size.0 + loc.0)
            {*screen_loc = character;}
        }
    }

    fn place_string
    (
        &mut self,
        string: &str,
        loc: &(usize, usize),
    )
    {
        for (i, c) in string.chars().enumerate()
        {
            self.place_char(c, &(loc.0+i, loc.1));
        }
    }

    fn clear(&mut self)
    {self.data = vec![' '; self.data.len()];}

    fn update_screen(&mut self)
    {
        self.clear();

        for (loc, element) in self.objects.world.clone()
        {
            let appearance = match element
            {
                WorldElement::Wall => APPEARANCE_WALL,
                WorldElement::Floor => APPEARANCE_FLOOR,
                WorldElement::Door => APPEARANCE_DOOR,
            };

            self.place_char(appearance, &(loc.0 as usize, loc.1 as usize));
        }

        for (entity, loc) in &self.objects.entities.clone()
        {
            match entity
            {
                EntityType::Cow => self.place_char(APPEARANCE_ENTITY_COW, &(loc.0 as usize, loc.1 as usize)),
            };
        }

        self.place_char
        (
            APPEARANCE_PLAYER, 
            &(self.objects.player.0 as usize,
            self.objects.player.1 as usize)
        );

        for i in 0..self.size.0
        {
            self.place_char(APPEARANCE_UI_BORDER_BOTTOM, &(i, self.size.1 - 2));
        }

        let msg = self.objects.message_for_player.clone();
        self.place_string(&msg, &(0, self.size.1 - 1));
    }

    fn update_screen_size(&mut self)
    {
        let new_terminal_size = term_size::dimensions().expect("could not get terminal dimentions");
        self.size = (new_terminal_size.0, new_terminal_size.1 - 1);
        self.data.resize(self.size.0 * self.size.1, ' ');
    }
}
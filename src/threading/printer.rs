extern crate term_size;

use std::{sync::mpsc, collections::HashMap};
use super::general::{ThreadMessage};
use super::world::WorldElement;

const APPEARANCE_PLAYER: char = '@';
const APPEARANCE_WALL: char = '#';
const APPEARANCE_FLOOR: char = '.';
const APPEARANCE_UI_BORDER_BOTTOM: char = '_';

pub fn routine(print_commands: mpsc::Receiver<super::general::ThreadMessage>)
{
    let mut screen = Screen
    {
        data: vec!(),
        objects: ScreenObjects::new(),
        size: (0usize, 0usize)
    };

    loop
    {
        let command = print_commands.recv().unwrap();

        screen.update_screen_size();

        match command
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Refresh => (),
                PrintCommand::PlayerUpdate(l) => screen.objects.player = l,
                PrintCommand::WorldUpdate(w) => screen.objects.world = w,
            }

            _ => panic!("Printer given unrecognizable command")
        }

        screen.update_screen();

        let output: String = screen.data.iter().collect();
        print!("{}\n", output);
    }
}

pub enum PrintCommand
{
    Refresh,
    PlayerUpdate((i32, i32)),
    WorldUpdate(HashMap<(i32, i32), WorldElement>)
}

struct Screen
{
    data: Vec<char>,
    objects: ScreenObjects,
    size: (usize, usize)
}

struct ScreenObjects
{
    player: (i32, i32),
    world: HashMap<(i32, i32), WorldElement>,
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
            message_for_player: String::new(),
        }
    }
}

impl Screen
{
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
            };

            self.place_char(appearance, &(loc.0 as usize, loc.1 as usize));
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
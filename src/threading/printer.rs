extern crate term_size;

use std::sync::{mpsc};
use super::general::{ThreadMessage};

const APPEARANCE_PLAYER: char = '@';

pub fn routine(print_commands: mpsc::Receiver<super::general::ThreadMessage>)
{
    let mut terminal_size = (0usize, 0usize);
    let mut screen_data: Vec<char> = vec!();
    let mut screen_objects = ScreenObjects::new();

    loop
    {
        let command = print_commands.recv().unwrap();

        let new_terminal_size = term_size::dimensions().expect("could not get terminal dimentions");
        terminal_size = (new_terminal_size.0, new_terminal_size.1 - 1);
        screen_data.resize(terminal_size.0 * terminal_size.1, ' ');

        match command
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Basic(s) => place_string(&mut screen_data, &terminal_size, s, 16, 0),
                PrintCommand::PlayerUpdate(l) => screen_objects.player = l,
            }

            _ => panic!("Printer given unrecognizable command")
        }

        screen_objects.update_screen(&mut screen_data, &terminal_size);

        let output: String = screen_data.iter().collect();
        print!("{}\n", output);
    }
}

pub enum PrintCommand
{
    Basic(String),
    PlayerUpdate((i16, i16)),
}

struct ScreenObjects
{
    player: (i16, i16),
}

impl ScreenObjects
{
    pub fn new() -> ScreenObjects
    {
        ScreenObjects
        {
            player: (0, 0),
        }
    }

    fn update_screen
    (
        &self,
        mut screen: &mut Vec<char>,
        screen_size: &(usize, usize),
    )
    {
        clear(screen);
        place_char
        (
            &mut screen, 
            screen_size, 
            APPEARANCE_PLAYER, 
            self.player.0.clone() as usize, 
            self.player.1.clone() as usize
        );
    }
}

fn place_char
(
    mut screen: &mut Vec<char>, 
    screen_size: &(usize, usize), 
    character: char, 
    x: usize, 
    y: usize
)
{
    if let Some(screen_loc) = screen.get_mut(y*screen_size.0 + x)
    {*screen_loc = character;}
}

fn place_string
(
    mut screen: &mut Vec<char>,
    screen_size: &(usize, usize),
    string: String,
    x: usize,
    y: usize
)
{
    for (i, c) in string.chars().enumerate()
    {
        place_char(&mut screen, screen_size, c, x+i, y)
    }
}

fn clear(screen: &mut Vec<char>)
{*screen = vec![' '; screen.len()];}
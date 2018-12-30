extern crate term_size;

use std::sync::{mpsc};
use super::general::{ThreadMessage};

pub fn routine(print_commands: mpsc::Receiver<super::general::ThreadMessage>)
{
    let mut terminal_size = (0usize, 0usize);
    let mut screendata: Vec<char> = vec!();

    loop
    {
        let command = print_commands.recv().unwrap();

        let new_terminal_size = term_size::dimensions().expect("could not get terminal dimentions");
        terminal_size = (new_terminal_size.0, new_terminal_size.1 - 1);
        screendata.resize(terminal_size.0 * terminal_size.1, ' ');

        match command
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Basic(s) => place_string(&mut screendata, &terminal_size, s, 16, 0)
            }

            _ => panic!("Printer given unrecognizable command")
        }


        let output: String = screendata.iter().collect();
        print!("{}\n", output);
    }
}

pub enum PrintCommand
{
    Basic(String),
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
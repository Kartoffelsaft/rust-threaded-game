extern crate term_size;

use std::sync::{mpsc};
use super::general::{ThreadMessage};

pub fn routine(print_commands: mpsc::Receiver<super::general::ThreadMessage>)
{
    let mut terminal_size = (0usize, 0usize);
    let mut screendata: Vec<char> = vec!();

    loop
    {
        let mut need_print = false;

        let new_terminal_size = term_size::dimensions().expect("could not get terminal dimentions");
        if new_terminal_size != terminal_size
        {
            terminal_size = (new_terminal_size.0, new_terminal_size.1 - 1);

            screendata = vec![' '; terminal_size.0 * terminal_size.1];

            need_print = true;
        } 

        let command = print_commands.recv().unwrap();
        match command
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Basic(s) => place_string(&mut screendata, &terminal_size, s, 16, 0)
            }

            _ => panic!("Printer given unrecognizable command")
        }

        if need_print
        {
            let output: String = screendata.iter().collect();
            println!("{}", output);
        }
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
    screen[screen_size.0 * y + x] = character;
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
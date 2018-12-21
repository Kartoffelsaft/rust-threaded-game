use std::sync::{mpsc};
use super::general::{ThreadMessage, PrintCommand};

pub fn routine(print_commands: mpsc::Receiver<super::general::ThreadMessage>)
{
    loop
    {
        let command = print_commands.recv().unwrap();
        match command
        {
            ThreadMessage::Printer(c) => 
            match c
            {
                PrintCommand::Basic(s) => println!("{}", s)
            }

            _ => panic!("Printer given unrecognizable command")
        }
    }
}
use std::sync::mpsc;
use super::general::{ThreadMessage};

pub fn routine(commands: mpsc::Receiver<ThreadMessage>)
{
    println!("player created");
}

pub enum PlayerCommand
{
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
}

struct Player
{
    location: (i16, i16),
}
use std::sync::mpsc;
use super::general::{ThreadMessage};

pub fn routine(commands: mpsc::Receiver<ThreadMessage>, teller: mpsc::Sender<ThreadMessage>)
{
    let mut player = Player::new();

    loop
    {
        match commands.recv().expect("player could not get message")
        {
            ThreadMessage::Player(c) => match c
            {
                PlayerCommand::Up(d) => 
                    for _ in 0..d { player.move_iter(true, false) },
                PlayerCommand::Down(d) =>
                    for _ in 0..d { player.move_iter(true, true) },
                PlayerCommand::Left(d) =>
                    for _ in 0..d { player.move_iter(false, false) },
                PlayerCommand::Right(d) =>
                    for _ in 0..d { player.move_iter(false, true) },
            }

            _ => panic!("player given unrecognizable command")          
        }

        teller.send
        (
            ThreadMessage::Printer
            (
                super::printer::PrintCommand::PlayerUpdate
                (
                    player.location
                )
            )
        ).expect("player could not send location");
    }
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

impl Player
{
    fn new() -> Player
    {
        Player
        {
            location: (1, 1),
        }
    }

    fn move_iter(&mut self, vertical: bool, positive: bool)
    {
        if vertical
        {
            if positive
            {self.location.1 += 1;}
            else
            {self.location.1 -= 1;}
        }
        else
        {
            if positive
            {self.location.0 += 1;}
            else
            {self.location.0 -= 1;}
        }
    }
}
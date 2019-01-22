use std::sync::mpsc;
use super::general::{ThreadMessage};
use std::collections::VecDeque;

pub fn routine(commands: mpsc::Receiver<ThreadMessage>, teller: mpsc::Sender<ThreadMessage>)
{
    let mut player = Player::new(commands, teller);

    loop
    {
        match player.commands.recv().expect("player could not get message")
        {
            ThreadMessage::Player(c) => match c
            {
                PlayerCommand::Move(m) => player.push_move(m),
                PlayerCommand::Collisions(c) => player.move_execute_physics(c),
            }

            _ => panic!("player given unrecognizable command")          
        }

        player.teller.send
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
    Move(Move),
    Collisions(Vec<(i32, i32)>),
}

pub enum Move
{
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
}

struct Player
{
    commands: mpsc::Receiver<ThreadMessage>,
    teller: mpsc::Sender<ThreadMessage>,

    location: (i32, i32),
    move_queue: VecDeque<Move>,
}

impl Player
{
    fn new(c: mpsc::Receiver<ThreadMessage>, t: mpsc::Sender<ThreadMessage>) -> Player
    {
        Player
        {
            commands: c,
            teller: t,

            location: (1, 1),
            move_queue: VecDeque::new(),
        }
    }

    fn push_move(&mut self, move_command: Move)
    {
        let move_to = match move_command
        {
            Move::Up(d) => (self.location.0, self.location.1 - d as i32),
            Move::Down(d) => (self.location.0, self.location.1 + d as i32),
            Move::Left(d) => (self.location.0 - d as i32, self.location.1),
            Move::Right(d) => (self.location.0 + d as i32, self.location.1),
        };

        self.teller.send
        (
            ThreadMessage::World
            (
                super::world::WorldCommand::CheckCollision
                (
                    self.location.clone(), 
                    move_to.clone(), 
                    super::world::WorldCollider::Player
                )
            )
        ).expect("player could not send message");

        self.move_queue.push_back(move_command);
    }

    fn move_execute_physics(&mut self, collisions: Vec<(i32, i32)>)
    {
        if collisions.len() == 0
        {
            let movement = self.move_queue.pop_front().expect("player has more collisions than moves");
            self.move_execute(movement);
        }
        else
        {
            self.teller.send
            (
                ThreadMessage::Printer
                (
                    super::printer::PrintCommand::MessageUpdate
                    (
                        String::from("Smack")
                    )
                )
            ).expect("player could not send smack");

            let mut dists: Vec<i32> = Vec::with_capacity(collisions.len());

            for collision in collisions
            {
                dists.push(((self.location.0 - collision.0) + (self.location.1 - collision.1)).abs())
            }

            let mut travel_dist = std::i32::MAX;
            for dist in dists
            {
                if travel_dist > dist
                {travel_dist = dist;}
            }
            travel_dist -= 1;

            let final_move = match self.move_queue.pop_front().expect("player has more collisions than moves")
            {
                Move::Up(_) => Move::Up(travel_dist as i16),
                Move::Down(_) => Move::Down(travel_dist as i16),
                Move::Left(_) => Move::Left(travel_dist as i16),
                Move::Right(_) => Move::Right(travel_dist as i16),
            };
            self.move_execute(final_move);
        }
    }

    fn move_execute(&mut self, movement: Move)
    {
        self.location = match movement
        {
            Move::Up(d) => (self.location.0, self.location.1 - d as i32),
            Move::Down(d) => (self.location.0, self.location.1 + d as i32),
            Move::Left(d) => (self.location.0 - d as i32, self.location.1),
            Move::Right(d) => (self.location.0 + d as i32, self.location.1),
        };
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
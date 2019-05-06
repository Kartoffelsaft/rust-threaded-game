use std::sync::mpsc::{Sender, Receiver};
use super::general::{ThreadMessage};
use super::collision_handler::{ptr::{CollDataPtr}, movement::{Direction, Moveable}};

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>, collider: CollDataPtr)
{
    let mut player = Player::new(commands, teller, collider);

    loop
    {
        match player.commands.recv().expect("player could not get message")
        {
            ThreadMessage::Player(c) => match c
            {
                PlayerCommand::Move(m) => 
                {
                    player.move_direction(m);
                    player.collider.set_player(player.location);
                },
            }

            ThreadMessage::BroadCast(_) => (),

            _ => panic!("player given unrecognizable command")          
        }

        player.teller.send
            (ThreadMessage::Printer
                (super::printer::PrintCommand::PlayerUpdate
                    (player.location
        ))).expect("player could not send location");
    }
}

#[derive(Debug)]
pub enum PlayerCommand
{
    Move(Direction),
}

struct Player
{
    commands: Receiver<ThreadMessage>,
    teller: Sender<ThreadMessage>,
    collider: CollDataPtr,

    location: (i32, i32),
}

impl Player
{
    fn new(c: Receiver<ThreadMessage>, t: Sender<ThreadMessage>, coll: CollDataPtr) -> Player
    {
        Player
        {
            commands: c,
            teller: t,
            collider: coll,

            location: (1, 1),
        }
    }

    /*
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
                (ThreadMessage::Printer
                    (super::printer::PrintCommand::MessageUpdate
                        (String::from("Smack")
            ))).expect("player could not send smack");

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

    fn _move_iter(&mut self, vertical: bool, positive: bool)
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
    */
}

impl Moveable for Player
{
    fn get_loc(&self) -> &(i32, i32)
    {&self.location}
    fn get_loc_mut(&mut self) -> &mut (i32, i32)
    {&mut self.location}

    fn get_collision_data_ptr(&self) -> CollDataPtr
    {self.collider.clone()}
}
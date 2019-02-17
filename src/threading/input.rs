use std::
{
    sync::mpsc::
    {
        Sender,
    },

    io::BufRead
};
use super::
{
    general::
    {
        ThreadMessage, 
        ThreadMessage::*
    }, 
    
    printer::PrintCommand, 
    
    player::
    {
        PlayerCommand::*, 
        Move::*
    }
};

pub fn routine(output: Sender<ThreadMessage>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();

    let mut last_input = String::new();

    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        inp = inp.trim().to_owned();

        output.send
            (ThreadMessage::Printer
                (super::printer::PrintCommand::Refresh
        )).expect("printer could not refresh screen");

        if inp == ""
        {
            inp = last_input.clone();
        }
        last_input = inp.clone();

        for msg in parse_input(inp)
        {
            output.send(msg).unwrap();
        }
    }
}

fn parse_input(input: String) -> Vec<ThreadMessage>
{
    let mut repeat_command_times = 1i16;
    let mut return_commands: Vec<ThreadMessage> = Vec::new();

    for word in input.split_whitespace()
    {
        match word.parse::<i16>()
        {
            Ok(i) => repeat_command_times = i,

            Err(_) =>
            {
                match word
                {
                    "up" | "u" => return_commands.push
                    (Player(Move(Up(repeat_command_times.clone())))),
                    
                    "down" | "d" => return_commands.push
                    (Player(Move(Down(repeat_command_times.clone())))),

                    "left" | "l" => return_commands.push
                    (Player(Move(Left(repeat_command_times.clone())))),

                    "right" | "r" => return_commands.push
                    (Player(Move(Right(repeat_command_times.clone())))),

                    "t" => return_commands.push
                    (World(super::world::WorldCommand::GenerateBuilding(((5, 3), (4, 4))))),

                    "e" => return_commands.push
                    (Entities(super::entities::EntitesCommand::Spawn)),

                    _ => (),
                }

                repeat_command_times = 1;
            }
        }
    }

    if return_commands.len() == 0
    {return_commands = vec!(Printer(PrintCommand::Refresh));}

    return_commands
}
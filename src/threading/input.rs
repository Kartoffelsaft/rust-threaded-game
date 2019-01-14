use std::sync::{mpsc};
use std::io::BufRead;
use super::{general::{ThreadMessage, ThreadMessage::*}, printer::PrintCommand, player::PlayerCommand::*};

pub fn routine(output: mpsc::Sender<ThreadMessage>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();

    let mut last_input = String::new();

    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        inp = inp.trim().to_owned();

        if inp == ""
        {inp = last_input.clone();}
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
                    (Player(Up(repeat_command_times.clone()))),
                    
                    "down" | "d" => return_commands.push
                    (Player(Down(repeat_command_times.clone()))),

                    "left" | "l" => return_commands.push
                    (Player(Left(repeat_command_times.clone()))),

                    "right" | "r" => return_commands.push
                    (Player(Right(repeat_command_times.clone()))),

                    _ => (),
                }

                repeat_command_times = 1;
            }
        }
    }

    if return_commands.len() == 0
    {return_commands = vec!(Printer(vec!(PrintCommand::Refresh)));}

    return_commands
}
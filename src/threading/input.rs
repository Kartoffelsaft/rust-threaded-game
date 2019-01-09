use std::sync::{mpsc};
use std::io::BufRead;
use super::{general::ThreadMessage, printer::PrintCommand, player::PlayerCommand::*};

pub fn routine(output: mpsc::Sender<ThreadMessage>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();
    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        inp = inp.trim().to_owned();
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
                    "up" => return_commands.push
                    (ThreadMessage::Player(Up(repeat_command_times.clone()))),
                    
                    "down" => return_commands.push
                    (ThreadMessage::Player(Down(repeat_command_times.clone()))),

                    "left" => return_commands.push
                    (ThreadMessage::Player(Left(repeat_command_times.clone()))),

                    "right" => return_commands.push
                    (ThreadMessage::Player(Right(repeat_command_times.clone()))),

                    _ => {},
                }

                repeat_command_times = 1;
            }
        }
    }

    return_commands
}
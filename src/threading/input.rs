use std::sync::{mpsc};
use std::io::BufRead;
use super::{general::ThreadMessage, printer::PrintCommand};

pub fn routine(output: mpsc::Sender<ThreadMessage>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();
    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        inp = inp.trim().to_owned();
        let player_test_loc = (inp.parse::<i16>().expect("not number"), 4i16);
        output.send(ThreadMessage::Printer(PrintCommand::PlayerUpdate(player_test_loc))).unwrap();
    }
}
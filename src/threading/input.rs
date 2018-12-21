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
        output.send(ThreadMessage::Printer(PrintCommand::Basic(inp))).unwrap();
    }
}
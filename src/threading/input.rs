use std::sync::mpsc;
use std::io::BufRead;
use super::general::ThreadMessage;

pub fn routine(output: mpsc::Sender<ThreadMessage>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();
    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        output.send(ThreadMessage::InputO(inp)).unwrap();
    }
}
use std::sync::mpsc;
use std::io::BufRead;

pub fn routine(output: mpsc::Sender<String>)
{
    let buffer_nl = std::io::stdin();
    let mut buffer = buffer_nl.lock();
    loop
    {
        let mut inp = String::new();
        buffer.read_line(&mut inp).unwrap();
        output.send(inp).unwrap();
    }
}
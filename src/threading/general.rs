use std::{thread, sync, collections::HashMap};

pub struct EveryThreadInstance
{
    interface: HashMap
    <
        &'static str,

        ThreadMetadata
    >,

    thread_output: sync::mpsc::Receiver<ThreadMessage>
}

impl EveryThreadInstance
{
    pub fn new_ptr() -> EveryThreadInstance
    {
        let (thread_output_s, thread_output_r) = sync::mpsc::channel();
        let mut new = EveryThreadInstance{interface: HashMap::new(), thread_output: thread_output_r};

        let read_input_s = thread_output_s.clone();
        let thread_input = thread::spawn(move || { super::input::routine(read_input_s); });
        new.interface.insert
        (
            "input", 
            
            ThreadMetadata
            {
                tell: None,
                finished: None,

                handle: thread_input,
            }
        );

        let (tell_printer_s, tell_printer_r) = sync::mpsc::channel();
        let thread_printer = thread::spawn(move || { super::printer::routine(tell_printer_r) });
        new.interface.insert
        (
            "printer", 
            
            ThreadMetadata
            {
                tell: Some(tell_printer_s),
                finished: None,

                handle: thread_printer,
            }
        );

        let (tell_player_s, tell_player_r) = sync::mpsc::channel();
        let thread_player = thread::spawn(move || { super::player::routine(tell_player_r) });
        new.interface.insert
        (
            "player", 
            
            ThreadMetadata
            {
                tell: Some(tell_player_s),
                finished: None,

                handle: thread_player,
            }
        );

        new
    }

    pub fn message_threads(&mut self)
    {            
        let source = self
            .thread_output
            .recv();

        match source
        {
            Result::Ok(message) => 
            {
                let reciever = match message
                {
                    ThreadMessage::Printer(_) => "printer",
                    ThreadMessage::Player(_) => "player",
                };

                self
                    .interface
                    .get(reciever)
                    .unwrap()
                    .tell
                    .as_ref()
                    .expect("thread does not have input")
                    .send(message)
                    .expect("send did not work");
            }

            Result::Err(e) => panic!("try recv failed: {}", e)
        }
    }
}

struct ThreadMetadata
{
    tell: Option//tell thread
        <sync::mpsc::Sender
            <ThreadMessage>>,                  

    finished: Option//thread finish loop
        <sync::mpsc::Receiver
            <bool>>,
    
    handle: thread::JoinHandle<()>,
}

pub enum ThreadMessage
{
    Printer(super::printer::PrintCommand),
    Player(super::player::PlayerCommand),
}

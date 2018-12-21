use std::{thread, sync, collections::HashMap};

pub struct EveryThreadInstance
{
    interface: HashMap
    <
        &'static str,

        ThreadMetadata
    >,
}

impl EveryThreadInstance
{
    pub fn new_ptr() -> EveryThreadInstance
    {
        let mut new = EveryThreadInstance{interface: HashMap::new()};

        let (read_input_s, read_input_r) = sync::mpsc::channel();
        let thread_input = thread::spawn(move || { super::input::routine(read_input_s); });
        new.interface.insert
        (
            "input", 
            
            ThreadMetadata
            {
                read: Some(read_input_r),
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
                read: None,
                tell: Some(tell_printer_s),
                finished: None,

                handle: thread_printer,
            }
        );

        new
    }

    // pub fn read(threads: &Arc<EveryThreadInstance>, thread: &str) -> Result<ThreadMessage, sync::mpsc::RecvError>
    // {
    //     let read_src =
    //     match &threads
    //         .interface
    //         .get(thread)
    //         .expect("thread does not exist")
    //         .read
    //     {
    //         Some(r) => r,
    //         None => panic!("thread does not have output"),
    //     };
    //     read_src.lock().unwrap().recv()
    // }

    // pub fn tell<'a>(threads: &'a Arc<sync::Mutex<EveryThreadInstance>>, thread: &str) -> &'a sync::Mutex<sync::mpsc::Sender<ThreadMessage>>
    // {
    //     let destination = 
    //     match &threads
    //         .lock()
    //         .unwrap()
    //         .interface
    //         .get(thread)
    //         .expect("thread does not exist")
    //         .tell
    //     {
    //         Some(r) => r,
    //         None => panic!("thread does not have input"),
    //     };

    //     destination     
    // }

    pub fn try_message_thread(&mut self, thread: &str)
    {            
        let source = self
            .interface
            .get(thread)
            .expect("thread does not exist")
            .read
            .as_ref()
            .expect("thread does not have output")
            .try_recv();

        match source
        {
            Result::Ok(message) => 
            {
                let reciever = match message
                {
                    ThreadMessage::Printer(_) => "printer"
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

            Result::Err(e) => match e
            {
                sync::mpsc::TryRecvError::Empty => {}
                
                _ => panic!("try recv failed: {}", e)
            }
        }
    }
}

struct ThreadMetadata
{
    read: Option//read from thread
        <sync::mpsc::Receiver
            <ThreadMessage>>,                        

    tell: Option//tell thread
        <sync::mpsc::Sender
            <ThreadMessage>>,                  

    finished: Option//thread finish loop
        <sync::mpsc::Receiver
            <bool>>,
    
    handle: thread::JoinHandle<()>,
}

impl ThreadMetadata
{
    pub fn new() -> ThreadMetadata
    {
        ThreadMetadata
        {
            read: None,
            tell: None,
            finished: None,

            handle: thread::spawn(move || {}),
        }
    }
}

pub enum ThreadMessage
{
    Printer(super::printer::PrintCommand),
}

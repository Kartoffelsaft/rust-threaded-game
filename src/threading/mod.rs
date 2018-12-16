mod input;

pub mod general
{
    use std::{thread, sync, collections::HashMap};

    pub struct EveryThreadInstance
    {
        interface: HashMap
        <
            &'static str,

            (
                Option<sync::mpsc::Receiver<String>>,
                Option<sync::mpsc::Sender<String>>, 
                thread::JoinHandle<()>,

            )
        >,
    }

    impl EveryThreadInstance
    {
        pub fn new() -> EveryThreadInstance
        {
            let mut interface_new = HashMap::new();

            let (from_input_s, from_input_r) = sync::mpsc::channel();
            let thread_input = thread::spawn(move || { super::input::routine(from_input_s); });
            interface_new.insert
            (
                "input", 
                (
                    Some(from_input_r), 
                    None,
                    thread_input
                )
            );

            let new = EveryThreadInstance{interface: interface_new};
            new
        }

        pub fn read(&mut self, thread: &str) -> Result<String, sync::mpsc::RecvError>
        {
            let read_src =
            match &self.interface
                .get(thread)
                .expect("thread does not exist")
                .0
            {
                Some(r) => r,
                None => panic!("thread does not have output"),
            };
            read_src.recv()
        }
    }
}
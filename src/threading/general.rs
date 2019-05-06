use std::
{
    thread, 
    sync::
    {
        mpsc::
        {
            Receiver,
            Sender,
            channel,
        },
    }, 
    collections::HashMap
};

pub struct EveryThreadInstance
{
    interface: HashMap
    <
        &'static str,

        ThreadMetadata
    >,

    thread_output: Receiver<ThreadMessage>
}

impl EveryThreadInstance
{
    pub fn new_ptr() -> EveryThreadInstance
    {
        let (thread_output_s, thread_output_r) = channel();
        let mut new = EveryThreadInstance{interface: HashMap::new(), thread_output: thread_output_r};
        let collision_data_ptr = super::collision_handler::ptr::CollDataPtr::new();

        let read_input_s = thread_output_s.clone();
        let thread_input = thread::spawn(move || { super::input::routine(read_input_s); });
        new.interface.insert
        (
            "input", 
            
            ThreadMetadata
            {
                tell: None,

                _handle: thread_input,
            }
        );

        let (tell_printer_s, tell_printer_r) = channel();
        let thread_printer = thread::spawn(move || { super::printer::routine(tell_printer_r) });
        new.interface.insert
        (
            "printer", 
            
            ThreadMetadata
            {
                tell: Some(tell_printer_s),

                _handle: thread_printer,
            }
        );

        let (tell_player_s, tell_player_r) = channel();
        let read_player_s = thread_output_s.clone();
        let collision_player = super::collision_handler::ptr::CollDataPtr::from(&collision_data_ptr);
        let thread_player = thread::spawn(move || { super::player::routine(tell_player_r, read_player_s, collision_player) });
        new.interface.insert
        (
            "player", 
            
            ThreadMetadata
            {
                tell: Some(tell_player_s),

                _handle: thread_player,
            }
        );

        let (tell_world_s, tell_world_r) = channel();
        let read_world_s = thread_output_s.clone();
        let collision_world = super::collision_handler::ptr::CollDataPtr::from(&collision_data_ptr);
        let thread_world = thread::spawn(move || { super::world::routine(tell_world_r, read_world_s, collision_world) });
        new.interface.insert
        (
            "world", 
            
            ThreadMetadata
            {
                tell: Some(tell_world_s),
                
                _handle: thread_world,
            }
        );

        let (tell_entities_s, tell_entities_r) = channel();
        let read_entities_s = thread_output_s.clone();
        let collision_entities = super::collision_handler::ptr::CollDataPtr::from(&collision_data_ptr);
        let thread_entities = thread::spawn(move || { super::entities::routine(tell_entities_r, read_entities_s, collision_entities) });
        new.interface.insert
        (
            "entities",

            ThreadMetadata
            {
                tell: Some(tell_entities_s),

                _handle: thread_entities,
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
                if let ThreadMessage::BroadCast(broadcast_message) = message
                {self.broadcast(broadcast_message);}
                else
                {
                    let reciever = match message
                    {
                        ThreadMessage::Printer(_) => "printer",
                        ThreadMessage::Player(_) => "player",
                        ThreadMessage::World(_) => "world",
                        ThreadMessage::Entities(_) => "entities",

                        ThreadMessage::BroadCast(_) => panic!("broadcast fell into else statement"),
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
            }

            Result::Err(e) => panic!("try recv failed: {}", e)
        }
    }

    fn broadcast(&mut self, message: BroadCastMessage)
    {
        for some_reciever in self.interface.values()
        {
            if let Some(reciever) = &some_reciever.tell
            {
                reciever.send(ThreadMessage::BroadCast(message.clone()));
            }
        }
    }
}

struct ThreadMetadata
{
    tell: Option//tell thread
        <Sender
            <ThreadMessage>>,
    
    _handle: thread::JoinHandle<()>,
}

#[derive(Debug)]
pub enum ThreadMessage
{
    Printer(super::printer::PrintCommand),
    Player(super::player::PlayerCommand),
    World(super::world::WorldCommand),
    Entities(super::entities::EntitesCommand),

    BroadCast(BroadCastMessage),
}

#[derive(Clone, Debug)]
pub enum BroadCastMessage
{
    Gametick,
}

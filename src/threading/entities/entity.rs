use std::
{
    sync::
    {
        mpsc::
        {
            Sender,
            Receiver,
            channel,
        },
        Arc,
        Mutex,
    },
    thread::
    {
        JoinHandle,
    },
};

use super::
{
    types::
    {
        cow,
    },
    super::
    {
        collision_handler::ptr::CollDataPtr,
    }
};

pub struct EntityCommunicator
{
    thread: JoinHandle<()>,

    pub teller: Sender<ToEntityCommand>,

    pub entity_inst: Arc<Mutex<Entity>>,
}

impl EntityCommunicator
{
    pub fn new(ptr: &CollDataPtr) -> EntityCommunicator
    {
        let (communicator_teller, entity_receiver) = channel();
        let new_entity_inst = Arc::new(Mutex::new(cow::Cow::new(ptr)));
        let new_entity_inst_cpy = new_entity_inst.clone();
        EntityCommunicator
        {
            thread: std::thread::spawn(move || 
            {
                routine(entity_receiver, new_entity_inst_cpy)
            }),
            teller: communicator_teller,
            entity_inst: new_entity_inst.clone(),
        }
    }
}

pub fn routine(commands: Receiver<ToEntityCommand>, mut entity_inst: Arc<Mutex<Entity>>)
{
    loop
    {
        parse_commands(&commands, &mut entity_inst);
    }
}

pub enum ToEntityCommand
{
    Update,
}

fn parse_commands(commands: &Receiver<ToEntityCommand>, entity_inst: &mut Arc<Mutex<Entity>>)
{
    let command = commands.recv().expect("entity could not get command");

    let mut entity_inst_deref = entity_inst.lock().unwrap();

    match command 
    {
        ToEntityCommand::Update => entity_inst_deref.update(),
    }
}

#[derive(Clone)]
pub enum EntityType
{
    Cow,
}

pub trait Entity: super::super::collision_handler::movement::Moveable
{
    fn update(&mut self);

    fn get_type(&self) -> EntityType;

    fn print_data(&self) -> (EntityType, (i32, i32))
    {(self.get_type(), self.get_loc().clone())}
}
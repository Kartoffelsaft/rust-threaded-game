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
    },
    thread::
    {
        JoinHandle,
    },
};

pub struct EntityCommunicator
{
    thread: JoinHandle<()>,

    pub teller: Sender<ToEntityCommand>,

    id: usize,
}

impl EntityCommunicator
{
    pub fn new(requester: &Sender<FromEntityCommand>, new_id: usize) -> EntityCommunicator
    {
        let (communicator_teller, entity_receiver) = channel();
        let requester_copy = requester.clone();
        EntityCommunicator
        {
            thread: std::thread::spawn(move || 
            {
                routine(entity_receiver, requester_copy)
            }),
            teller: communicator_teller,
            id: new_id,
        }
    }
}

pub fn routine(commands: Receiver<ToEntityCommand>, teller: Sender<FromEntityCommand>)
{
    let mut entity_inst = Entity::new(commands, teller);

    loop
    {
        entity_inst.update();

        entity_inst.parse_commands();
    }
}

pub enum ToEntityCommand
{
    Update,
}

pub enum FromEntityCommand
{
    Update((EntityType, (i32, i32))),
}

struct Entity
{
    commands: Receiver<ToEntityCommand>,
    teller: Sender<FromEntityCommand>,

    loc: (i32, i32),
    e_type: EntityType,
}

#[derive(Clone)]
pub enum EntityType
{
    Cow,
}

impl Entity
{
    pub fn new(c: Receiver<ToEntityCommand>, t: Sender<FromEntityCommand>) -> Entity
    {
        Entity
        {
            commands: c,
            teller: t,

            loc: (10, 10),
            e_type: EntityType::Cow,
        }
    }

    pub fn parse_commands(&mut self)
    {
        let command = self.commands.recv().expect("entity could not get command");

        match command 
        {
            ToEntityCommand::Update => (),
        }
    }

    pub fn update(&mut self)
    {
        self.teller.send(FromEntityCommand::Update((self.e_type.clone(), self.loc))).expect("entity could not send update");
    }
}
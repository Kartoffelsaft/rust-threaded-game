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

pub struct EntityCommunicator
{
    thread: JoinHandle<()>,

    pub teller: Sender<ToEntityCommand>,

    pub entity_inst: Arc<Mutex<Entity>>,
}

impl EntityCommunicator
{
    pub fn new() -> EntityCommunicator
    {
        let (communicator_teller, entity_receiver) = channel();
        let new_entity_inst = Arc::new(Mutex::new(Entity::new()));
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

pub struct Entity
{
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
    pub fn new() -> Entity
    {
        Entity
        {
            loc: (10, 10),
            e_type: EntityType::Cow,
        }
    }

    pub fn update(&mut self)
    {}

    pub fn print_data(&self) -> (EntityType, (i32, i32))
    {(self.e_type.clone(), self.loc.clone())}
}
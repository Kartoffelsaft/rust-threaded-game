use std::
{
    sync::
    {
        mpsc::
        {
            Receiver,
            Sender,
            channel,
        },
    },
};
use super::
{
    general::
    {
        ThreadMessage,
    },
    collision_handler::ptr::CollDataPtr,
};

pub mod entity;
mod types;

const _ENTITY_TICK_MILLIS: usize = 10;

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>, collider: CollDataPtr)
{
    let mut metaentity = Entities::new(commands, teller, collider);

    loop
    {
        metaentity.printer_update();

        metaentity.parse_commands();
    }
}

pub enum EntitesCommand
{
    Spawn,
}

struct Entities
{
    commands: Receiver<ThreadMessage>,
    teller: Sender<ThreadMessage>,

    collider: CollDataPtr,

    ents: Vec<entity::EntityCommunicator>,
}

impl Entities
{
    pub fn new(c: Receiver<ThreadMessage>, t: Sender<ThreadMessage>, coll: CollDataPtr) -> Entities
    {
        Entities
        {
            commands: c,
            teller: t,

            collider: coll,

            ents: vec!(),
        }
    }

    pub fn new_entity(&mut self)
    {
        self.ents.push
            (entity::EntityCommunicator::new(&self.collider));
    }

    pub fn command_entities_to_update(&mut self)
    {
        for ent in &mut self.ents
        {
            ent.teller.send(entity::ToEntityCommand::Update).expect("metaentity could not tell entity to update");
        }
    }

    pub fn parse_commands(&mut self)
    {
        match self.commands.recv().expect("metaentity could not get commands")
        {
            ThreadMessage::Entities(ec) => match ec
            {
                EntitesCommand::Spawn => self.new_entity(),
            },
            _ => panic!("metaentity given unrecognizable command"),
        };
    }

    fn printer_update(&mut self)
    {
        let mut ent_updates_buffer: Vec<(entity::EntityType, (i32, i32))> = Vec::with_capacity(self.ents.len());
        for ent in &self.ents
        {
            let entity_inst_deref = ent.entity_inst.lock().unwrap();
            ent_updates_buffer.push(entity_inst_deref.print_data());
        }

        self.teller.send
            (super::general::ThreadMessage::Printer
                (super::printer::PrintCommand::EntitiesUpdate(ent_updates_buffer)
            )
        ).expect("metaentity could not send print information");
    }
}






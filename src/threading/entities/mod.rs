use std::
{
    sync::
    {
        mpsc::
        {
            Receiver,
            Sender,
            channel,

            RecvTimeoutError,
        },
    },

    time::Duration,
    ops::Deref,
};
use super::
{
    general::
    {
        ThreadMessage,
        BroadCastMessage,
    },
    collision_handler::ptr::CollDataPtr,
};

pub mod entity;
mod types;

const ENTITY_TICK_MILLIS: usize = 10;

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>, collider: CollDataPtr)
{
    let mut metaentity = Entities::new(commands, teller, collider);

    loop
    {
        if metaentity.collider_update()
        {metaentity.printer_update();}

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
    prev_ents_coll: Vec<(i32, i32)>,
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
            prev_ents_coll: vec!(),
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
        match self.commands.recv_timeout(Duration::from_millis(ENTITY_TICK_MILLIS as u64))
        {
            Ok(c) => match c
            {
                ThreadMessage::Entities(ec) => match ec
                {
                    EntitesCommand::Spawn => self.new_entity(),
                },

                ThreadMessage::BroadCast(bc) => match bc
                {
                    BroadCastMessage::Gametick => self.command_entities_to_update()
                }

                _ => panic!("metaentity given unrecognizable command"),
            }

            Err(e) => match e
            {
                RecvTimeoutError::Timeout => (),

                RecvTimeoutError::Disconnected => panic!("metaentity recv disconnected"),
            }
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

    fn collider_update(&mut self) -> bool
    {
        let mut entities: Vec<(i32, i32)> = vec!();
        
        for ent in &self.ents
        {
            entities.push
            (
                ent
                    .entity_inst
                    .lock()
                    .expect("metaentity could not lock entity")
                    .deref()
                    .get_loc()
                    .clone()
            );
        }

        let mut collision_change = self.prev_ents_coll.len() != entities.len();
        
        if !collision_change
        {
            for (new_ent, prev_ent) in entities.iter().zip(self.prev_ents_coll.iter())
            {
                if new_ent != prev_ent
                {
                    collision_change = true;
                    break;
                }
            }
        }

        if collision_change
        {
            self.prev_ents_coll = entities.clone();
            self.collider.set_entities(entities);
        }

        collision_change
    }
}






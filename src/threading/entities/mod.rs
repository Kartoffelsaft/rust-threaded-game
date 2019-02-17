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

    time::
    {
        Duration,
    },
};
use super::
{
    general::
    {
        ThreadMessage,
    }
};

const ENTITY_TICK_MILLIS: usize = 10;

pub fn routine(commands: Receiver<ThreadMessage>, teller: Sender<ThreadMessage>)
{
    let mut metaentity = Entities::new(commands, teller);

    loop
    {
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

    ent_requests: Receiver<entity::FromEntityCommand>,
    ent_teller_master_copy: Sender<entity::FromEntityCommand>,

    next_id: usize,
    ents: Vec<entity::EntityCommunicator>,
}

impl Entities
{
    pub fn new(c: Receiver<ThreadMessage>, t: Sender<ThreadMessage>) -> Entities
    {
        let (e_t_m_c, e_r) = channel();

        Entities
        {
            commands: c,
            teller: t,

            ent_requests: e_r,
            ent_teller_master_copy: e_t_m_c,

            next_id: 0,
            ents: vec!(),
        }
    }

    pub fn new_entity(&mut self)
    {
        self.ents.push
            (entity::EntityCommunicator::new
                (&self.ent_teller_master_copy, 
                self.next_id
        ));
        self.next_id += 1;
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
        let mut ent_updates_buffer = vec!();
        for c in self.ent_requests.try_iter() 
        {
            match c
            {
                entity::FromEntityCommand::Update(ec) => 
                {
                    ent_updates_buffer.push(ec)
                },
            }
        }
        if ent_updates_buffer.len() > 0
        {
            self.teller.send
                (super::general::ThreadMessage::Printer
                    (super::printer::PrintCommand::EntitiesUpdate(ent_updates_buffer)
                )
            ).expect("metaentity could not send print information");
        }

        match self.commands.recv_timeout(Duration::from_millis(ENTITY_TICK_MILLIS as u64))
        {
            Ok(c) => match c
            {
                ThreadMessage::Entities(ec) => match ec
                {
                    EntitesCommand::Spawn => self.new_entity(),
                },

                _ => panic!("metaentity given unrecognizable command"),
            },

            Err(e) => match e
            {
                RecvTimeoutError::Timeout => (),

                RecvTimeoutError::Disconnected => panic!("metaentity could not get commands")
            },
        };
    }
}

pub mod entity;





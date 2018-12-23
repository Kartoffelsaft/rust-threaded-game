mod threading;

pub fn start()
{
    let mut threads = threading::general::EveryThreadInstance::new_ptr();
    
    loop
    {
        threads.message_threads();
    }
}
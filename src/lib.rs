mod threading;

pub fn start()
{
    let mut threads = threading::general::EveryThreadInstance::new();
    let input_test = match threads.read("input").unwrap()
    {
        threading::general::ThreadMessage::InputO(s) => s,
    };
    println!("{}", input_test);
}
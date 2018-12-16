mod threading;

pub fn start()
{
    let mut threads = threading::general::EveryThreadInstance::new();
    println!("{}", threads.read("input").unwrap());
}
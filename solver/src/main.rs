fn main() {
    println!("Environment variables:");
    std::env::vars().for_each(|(name, value)| {
        println!("{}={}", name, value);
    });

    println!("Command line arguments:");
    std::env::args().for_each(|arg| {
        println!("{}", arg);
    });
}

use std::io;
use std::sync::mpsc;
use std::thread;
enum Commands {
    TRANSFER,
    VIEWBALANCE,
    VIEWALL,
    HELP,
    ERR,
}

//read line from STD-IN
fn readline() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().split(' ').map(|s| s.to_string()).collect();
}

/*pub(crate) fn get_input() -> Commands {
    let line = readline();
    return match line[0].as_str() {
        "transfer" => Commands::TRANSFER,
        "viewbalance" => Commands::VIEWBALANCE,
        "viewall" => Commands::VIEWALL,
        "help" => Commands::HELP,
        _ => Commands::ERR
    };
}*/

//thread read lines from stdin and send them in tunnel, return tunnel to caller
pub(crate) fn init() -> mpsc::Receiver<Vec<String>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            tx.send(readline()); //frontend.rs
        }
    });
    return rx;
}

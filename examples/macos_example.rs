fn main() {
    let arg = std::env::args().nth(1);
    let port = match arg {
        Some(filename) => {
            println!("{}", filename);
            serial::open(&filename)
        }
        None => {
            eprintln!("usage: macos_example <TTY path>");
            std::process::exit(1);
        }
    }
    .unwrap();
    let mut can = slcan::CanSocket::<serial::SystemPort>::new(port);

    can.close().unwrap();
    can.open(slcan::BitRate::Setup1Mbit).unwrap();

    loop {
        match can.read() {
            Ok(frame) => println!("{}", frame),
            Err(error) => match error.kind() {
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => (),
                _ => eprintln!("{:?}", error),
            },
        }
    }
}

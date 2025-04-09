use embedded_can::nb::Can;

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
        match can.receive() {
            Ok(frame) => println!("{}", frame),
            Err(nb::Error::WouldBlock) => (),
            Err(nb::Error::Other(error)) => match error.inner().kind() {
                std::io::ErrorKind::TimedOut => (),
                _ => eprintln!("{:?}", error),
            },
        }
    }
}

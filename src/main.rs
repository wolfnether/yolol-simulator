use std::io::stdin;
use std::io::BufRead;

use clap::App;
use clap::Arg;
use yolol_devices::Networks;
use yolol_runner::YololRunner;

fn main() {
    let matches = App::new("Yolol simulator")
        .arg(
            Arg::with_name("network_file")
                .short("n")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let network_file = matches.value_of("network_file").unwrap();
    if let Some(networks) = &mut Networks::<YololRunner>::deserialize(network_file) {
        let mut buffer = String::new();
        let mut old_buffer = String::new();
        while stdin().lock().read_line(&mut buffer).is_ok() {
            buffer = buffer.replace("\r\n", "\n").replace("\n", "");
            if buffer.is_empty() {
                buffer = old_buffer
            }
            match buffer.as_str() {
                "p" | "parse" => networks.parse_all_chip_file(),
                "n" | "next" => networks.step(),
                "g" | "globals" => networks.print_globals(),
                "" => (),
                v => println!("unknown command: {}", v),
            }
            old_buffer = buffer;
            buffer = String::new();
        }
        println!("{:?}", networks);
    } else {
        panic!("something wrong happened");
    }
}

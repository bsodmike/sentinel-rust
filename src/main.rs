extern crate futures;
extern crate tokio;

#[tokio::main]
async fn main() {
    let enable_cli_options: bool =
        sentinel::configure::fetch::<bool>(String::from("cli_options")).unwrap_or(false);

    // Load options from CLI
    if enable_cli_options {
        let _conf = sentinel::opts::parse_args().unwrap();
        println!("Conf: {}", _conf);
    }

    sentinel::run().await;
}

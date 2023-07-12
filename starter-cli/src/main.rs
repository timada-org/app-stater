use clap::{arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand(
            Command::new("migrate")
                .about("Migrate database schema to latest")
                .arg(arg!(-c --config <CONFIG>).required(false)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("migrate", _sub_matches)) => {
            println!("Migration database....");
        }
        Some(("reset", _sub_matches)) => {
            println!("Reset database...");
        }
        _ => unreachable!(),
    };
}

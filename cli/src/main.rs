use clap::{value_parser, Arg, ArgAction, Command};
use std::str::FromStr;
use tracing::{error, Level};

#[tokio::main]
async fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("log")
                .long("log")
                .help("Log level")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set),
        )
        .subcommand(Command::new("migrate").about("Migrate database schema to latest"))
        .subcommand(Command::new("serve").about("Start starter server using bin"))
        .get_matches();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(
            matches
                .get_one::<String>("log")
                .map(|log| Level::from_str(log).expect("failed to deserialize log"))
                .unwrap_or(Level::ERROR),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    match matches.subcommand() {
        Some(("migrate", _sub_matches)) => {
            println!("Migration database...");
        }
        Some(("reset", _sub_matches)) => {
            println!("Reset database...");
        }
        Some(("serve", _sub_matches)) => {
            let res = tokio::try_join! {
                starter_api::serve(),
                starter_app::serve()
            };

            if let Err(e) = res {
                error!("{}", e);
                std::process::exit(1);
            }
        }
        _ => unreachable!(),
    };
}

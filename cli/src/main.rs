use clap::{value_parser, Arg, ArgAction, Command};
use std::str::FromStr;
use tracing::error;
use tracing_subscriber::{prelude::*, EnvFilter};

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
        .subcommand(Command::new("serve").about("Start starter server using bin"))
        .get_matches();

    let log = matches
        .get_one::<String>("log")
        .map(|s| s.to_owned())
        .unwrap_or("error".to_owned());

    let env_filter = match EnvFilter::from_str(log.as_str()) {
        Ok(filter) => filter,
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .init();

    match matches.subcommand() {
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

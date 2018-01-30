extern crate doxidize;

#[macro_use]
extern crate configure;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use slog::Drain;
use structopt::StructOpt;

use doxidize::Config;

#[derive(StructOpt, Debug)]
#[structopt(name = "doxidize", about = "Execllent documentation tooling for Rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "build")]
    Build,
    #[structopt(name = "publish")]
    Publish,
    #[structopt(name = "serve")]
    Serve,
}

fn main() {
    use_default_config!();

    let doxidize_version = env!("CARGO_PKG_VERSION");

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => doxidize_version));

    let config = Config::default();

    match Opt::from_args() {
        Opt { ref command } if command.is_some() => {
            // we just checked that it's Some
            match command.as_ref().unwrap() {
                &Command::Build => {
                    doxidize::ops::build(&config, &log).expect("could not build docs");
                },
                &Command::Publish => {
                    doxidize::ops::publish(&config, &log).expect("could not publish docs");
                },
                &Command::Serve => {
                    doxidize::ops::serve(&config, &log).expect("could not serve docs");
                },
            }
        }
        _ => {
            // default with no command
            doxidize::ops::create_skeleton(&config, &log).expect("could not create skeleton");
        }
    }
}

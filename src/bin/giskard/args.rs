use std::path::PathBuf;

use clap::AppSettings;

pub struct Args {
    pub cfg_file: Option<PathBuf>,
    pub taskfile: Option<String>,
    pub command: Command,
}

pub enum Command {
    Ls(::commands::ls::Args),
}

pub fn get() -> Args {
    let matches = clap_app!(giskard =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "the todo.txt butler")
        (@arg CONFIG: -c --config +takes_value "Path of the config file to use [$XDG_CONFIG_HOME/giskard/config.toml].")
        (@arg TASKFILE: -t --taskfile +takes_value "Taskfile to operate on if several are defined [defaults to using the first].")
        (@subcommand ls =>
            (about: "list the current tasks")
        )
    ).setting(AppSettings::SubcommandRequired)
        .get_matches();

    let command = match matches.subcommand() {
        ("ls", Some(_sub_m)) => Command::Ls(::commands::ls::Args {}),
        _ => unreachable!(),
    };

    Args {
        cfg_file: matches.value_of("CONFIG").map(Into::into),
        taskfile: matches.value_of("TASKFILE").map(Into::into),
        command: command,
    }
}

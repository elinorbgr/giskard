#[macro_use]
extern crate clap;
extern crate giskard;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

mod args;
mod commands;
mod config;

fn main() {
    let args = args::get();
    let config = match config::read(args.cfg_file.as_ref().map(|p| p.as_path())) {
        Ok(cfg) => cfg,
        Err(e) => {
            config::report_error(args.cfg_file.as_ref().map(|p| p.as_path()), e);
            return;
        }
    };

    let task_config = if let Some(ref name) = args.taskfile {
        match config.taskfiles.iter().find(|t| &t.name == name) {
            Some(task_config) => task_config.clone(),
            None => {
                println!(
                    "There is no taskfile named `{}` in giskard configuration.",
                    name
                );
                return;
            }
        }
    } else {
        config.taskfiles[0].clone()
    };

    let task_file = match giskard::TaskFile::open(
        &task_config.task_file,
        task_config.done_file.as_ref().or_else(|| {
            if !task_config.discard_done {
                Some(&task_config.task_file)
            } else {
                None
            }
        }),
    ) {
        Ok(f) => f,
        Err(e) => {
            println!(
                "Failed to open task file `{}`",
                task_config.task_file.to_string_lossy()
            );
            println!("Error was: {}", e);
            return;
        }
    };

    match args.command {
        ::args::Command::Ls(args) => ::commands::ls::run(task_file, args),
    }
}

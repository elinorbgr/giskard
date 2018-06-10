use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub taskfiles: Vec<TaskFileConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskFileConfig {
    pub name: String,
    pub task_file: PathBuf,
    pub done_file: Option<PathBuf>,
    #[serde(default)]
    pub discard_done: bool,
}

pub enum Error {
    Malformed(::toml::de::Error),
    NoConfig,
    Io(io::Error),
}

fn get_xdg_path() -> Option<PathBuf> {
    let xdg_dirs = ::xdg::BaseDirectories::with_prefix("giskard").ok()?;
    xdg_dirs.find_config_file("config.toml")
}

pub fn read(file: Option<&Path>) -> Result<Config, Error> {
    let cfg_text = if let Some(file) = file {
        read_to_string(file).map_err(Error::Io)?
    } else {
        match get_xdg_path() {
            Some(path) => read_to_string(path).map_err(Error::Io)?,
            None => return Err(Error::NoConfig),
        }
    };

    ::toml::from_str(&cfg_text).map_err(Error::Malformed)
}

pub fn report_error(path: Option<&Path>, e: Error) {
    print!("Error loading giskard configuration: ");
    match e {
        Error::Malformed(err) => {
            if let Some(p) = path {
                println!("could not parse file `{}`.", p.to_string_lossy());
            } else {
                println!(
                    "could not parser file `{}`.",
                    get_xdg_path().unwrap().to_string_lossy()
                );
            }
            println!("Error was: {}", err);
        }
        Error::NoConfig => {
            println!("could not find a configuration file.");
            println!(
                "Create it at `$XDG_CONFIG_HOME/giskard/config.toml` or pass a --config argument."
            )
        }
        Error::Io(err) => {
            if let Some(p) = path {
                println!("could not read file `{}`.", p.to_string_lossy());
            } else {
                println!(
                    "could not read file `{}`.",
                    get_xdg_path().unwrap().to_string_lossy()
                );
            }
            println!("Error was: {}", err);
        }
    }
}

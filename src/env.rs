use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// The method by which the userscript was launched, either `hints` (started via hints)
/// or `command` (started via command or key binding).
#[derive(Clone, Debug)]
pub enum SpawnMode {
    /// Indicates that the userscript was started via hints.
    Hints(HintsVars),
    /// Indicates that the userscript was started via command or key binding.
    Command(CommandVars),
}

const MODE: &str = "QUTE_MODE";

/// Returns [`SpawnMode`] based on environment variable `QUTE_MODE`.
///
/// [`SpawnMode`]: ./enum.SpawnMode.html
#[inline]
pub fn mode() -> SpawnMode {
    match unwrap_env(MODE).as_str() {
        "hints" => SpawnMode::Hints(HintsVars),
        "command" => SpawnMode::Command(CommandVars),
        _ => panic!("invalid {} variable", MODE),
    }
}

/// Struct with methods for [`SpawnMode::Hints`]-specific variables.
///
/// [`SpawnMode::Hints`]: ./enum.SpawnMode.html#variant.Hints
#[derive(Clone, Debug)]
pub struct HintsVars;

const HINTS_URL: &str = "QUTE_URL";
const HINTS_SELECTED_TEXT: &str = "QUTE_SELECTED_TEXT";
const HINTS_SELECTED_HTML: &str = "QUTE_SELECTED_HTML";

impl HintsVars {
    /// Returns the URL selected via hints.
    #[inline]
    pub fn url() -> String {
        unwrap_env(HINTS_URL)
    }

    /// Returns the plain text of the element selected via hints.
    #[inline]
    pub fn selected_text(&self) -> String {
        unwrap_env(HINTS_SELECTED_TEXT)
    }

    /// Returns the HTML of the element selected via hints.
    #[inline]
    pub fn selected_html(&self) -> String {
        unwrap_env(HINTS_SELECTED_HTML)
    }
}

/// Struct with methods for [`SpawnMode::Command`]-specific variables.
///
/// [`SpawnMode::Command`]: ./enum.SpawnMode.html#variant.Command
#[derive(Clone, Debug)]
pub struct CommandVars;

const COMMAND_URL: &str = "QUTE_URL";
const COMMAND_TITLE: &str = "QUTE_TITLE";
const COMMAND_SELECTED_TEXT: &str = "QUTE_SELECTED_TEXT";
const COMMAND_COUNT: &str = "QUTE_COUNT";

impl CommandVars {
    /// Returns the URL of the current page.
    #[inline]
    pub fn url() -> String {
        unwrap_env(COMMAND_URL)
    }

    /// Returns the title of the current page.
    #[inline]
    pub fn title(&self) -> String {
        unwrap_env(COMMAND_TITLE)
    }

    /// Returns the text currently selected on the page.
    #[inline]
    pub fn selected_text(&self) -> String {
        unwrap_env(COMMAND_SELECTED_TEXT)
    }

    /// Returns the `count` from the spawn command running the userscript.
    #[inline]
    pub fn count(&self) -> String {
        unwrap_env(COMMAND_COUNT)
    }
}

const USER_AGENT: &str = "QUTE_USER_AGENT";

/// Returns the currently set user agent string.
#[inline]
pub fn user_agent() -> String {
    unwrap_env(USER_AGENT)
}

const HTML: &str = "QUTE_HTML";

/// Returns the path of a file containing the HTML source of the current page.
#[inline]
pub fn html() -> PathBuf {
    unwrap_env(HTML).into()
}

const TEXT: &str = "QUTE_TEXT";

/// Returns the path of a file containing the plain text of the current page.
#[inline]
pub fn text() -> PathBuf {
    unwrap_env(TEXT).into()
}

/// FIFO file to write commands to.
#[derive(Clone, Debug)]
pub struct Fifo {
    path: PathBuf,
}

impl Fifo {
    #[inline]
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Fifo {
            path: path.as_ref().into(),
        }
    }

    /// Open the `FIFO` as a file.
    #[inline]
    pub fn file(&self) -> Result<File, io::Error> {
        File::open(&self.path)
    }

    /// Write a string to the `FIFO` file.
    ///
    /// On Unix/macOS, this is a named pipe and commands written to it will get executed
    /// immediately.
    /// On Windows, this is a regular file, and the commands in it will be executed as
    /// soon as your userscript terminates.
    #[inline]
    pub fn write(&self, message: &str) -> Result<(), io::Error> {
        let mut file = self.file()?;
        file.write_all(message.as_bytes())
    }
}

const FIFO: &str = "QUTE_FIFO";

/// Returns an instance of [`Fifo`] based on the environment variable `QUTE_FIFO`.
///
/// [`Fifo`]: ./struct.Fifo.html
#[inline]
pub fn fifo() -> Fifo {
    let fifo_str = unwrap_env(FIFO);
    Fifo::new(fifo_str)
}

const CONFIG_DIR: &str = "QUTE_CONFIG_DIR";

/// Returns the path of the directory containing qutebrowser's configuration.
#[inline]
pub fn config_dir() -> PathBuf {
    unwrap_env(CONFIG_DIR).into()
}

const DATA_DIR: &str = "QUTE_DATA_DIR";

/// Returns the path of the directory containing qutebrowser's data.
#[inline]
pub fn data_dir() -> PathBuf {
    unwrap_env(DATA_DIR).into()
}

const DOWNLOAD_DIR: &str = "QUTE_DOWNLOAD_DIR";

/// Returns the path of the downloads directory.
#[inline]
pub fn download_dir() -> PathBuf {
    unwrap_env(DOWNLOAD_DIR).into()
}

const COMMANDLINE_TEXT: &str = "QUTE_COMMANDLINE_TEXT";

/// Returns the text in qutebrowser's command line.
#[inline]
pub fn commandline_text() -> String {
    unwrap_env(COMMANDLINE_TEXT)
}

#[inline]
fn unwrap_env(key: &str) -> String {
    env::var(key).expect(&format!("variable {} not set", key))
}

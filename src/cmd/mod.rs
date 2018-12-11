mod missing;
mod outdated;

use std::path::Path;
use xtodo::Result;

pub enum Cmd {
    Missing,
    Outdated,
}

pub fn run_command(command: &Cmd, track_dir: &Path, spec_dir: Option<&Path>) -> Result<()> {
    match command {
        Cmd::Missing => missing::list_missing_exercises(track_dir, spec_dir),
        Cmd::Outdated => outdated::list_outdated_exercises(track_dir, spec_dir),
    }
}

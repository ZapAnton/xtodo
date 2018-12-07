mod missing;
mod outdated;

use std::path::Path;

pub fn list_missing_exercises(track_dir: &Path) -> xtodo::Result<()> {
    missing::list_missing_exercises(track_dir)
}

pub fn list_outdated_exercises(track_dir: &Path) -> xtodo::Result<()> {
    outdated::list_outdated_exercises(track_dir)
}

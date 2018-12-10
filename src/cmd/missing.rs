use serde_json::Value;
use std::{collections::HashSet, path::Path};

fn get_existing_exercises(spec_dir: Option<&Path>) -> xtodo::Result<HashSet<String>> {
    let exercise_names = if let Some(spec_dir) = spec_dir {
        spec_dir
            .join("exercises")
            .read_dir()?
            .filter(|dir_entry| dir_entry.is_ok())
            .map(|exercise_dir| exercise_dir.unwrap().file_name().into_string().unwrap())
            .collect()
    } else {
        let existing_exercise_dirs: Value = reqwest::get(
            "https://api.github.com/repos/exercism/problem-specifications/contents/exercises/",
        )?
        .json()?;

        existing_exercise_dirs
            .as_array()
            .unwrap()
            .iter()
            .map(|exercise_dir| {
                exercise_dir
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect()
    };

    Ok(exercise_names)
}

fn get_implemented_exercises(config: &Value) -> HashSet<String> {
    config
        .get("exercises")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|exercise| exercise.get("slug").unwrap().as_str().unwrap().to_string())
        .collect()
}

pub fn list_missing_exercises(track_dir: &Path, spec_dir: Option<&Path>) -> xtodo::Result<()> {
    let config = xtodo::get_config_value(track_dir)?;

    let track_name = config.get("language").unwrap().as_str().unwrap();

    let implemented_exercises = get_implemented_exercises(&config);

    let existing_exercises = get_existing_exercises(spec_dir)?;

    let unimplemented_exercises: Vec<String> = existing_exercises
        .difference(&implemented_exercises)
        .map(|unimplemented_exercise| unimplemented_exercise.to_owned())
        .collect();

    println!(
        "Unimplemented exercises for the {} track:\n{}",
        track_name,
        unimplemented_exercises
            .iter()
            .enumerate()
            .map(|(idx, unimplemented_exercise)| format!("{}) {}", idx + 1, unimplemented_exercise))
            .collect::<Vec<String>>()
            .join("\n")
    );

    Ok(())
}

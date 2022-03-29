use std::fs::File;
use serde_json::Result;

// Reading the dataset
fn read_dataset(path: String) -> Result<HashMap<String, User>> {
    let file = File::open(path).expect("Failed to load file");
    let reader = BufReader::new(file);
    let dataset: HashMap<String, User> = serde_json::from_reader(reader)?;

    Ok(dataset)
}
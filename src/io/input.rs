use
{
	serde::{Deserialize, Serialize},
	std::{fs, error::Error, path::Path},
};

/// # Summary
pub fn from_editor<'de, T>(entity: T) -> Result<String, Box<dyn Error>> where
	T : Deserialize<'de> + Serialize
{
	let temp_file_stem = edit::Builder::new().tempfile()?;
	let temp_path_str = format!("{}.toml", temp_file_stem.path().to_string_lossy());
	let temp_path = Path::new(&temp_path_str);

	fs::write(temp_path, toml::to_string_pretty(&entity)?)?;
	edit::edit_file(&temp_path)?;
	return Ok(toml::from_slice(&fs::read(&temp_path)?)?);
}

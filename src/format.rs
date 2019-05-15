
pub enum Format {
  Text,
  JSON
}

impl Format {
  pub fn from_string(f: String) -> Format {
    if f.trim().to_lowercase() == "json" {
      return Format::JSON;
    }

    Format::Text
  }
}
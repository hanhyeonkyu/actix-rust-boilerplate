use serde::Deserialize;

#[derive(Deserialize)]
pub struct IEcho {
  pub name: String,
}

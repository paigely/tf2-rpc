pub use anyhow::Result;
use core::fmt;

#[derive(Debug, Clone)]
pub enum Gamemode {
	Casual,
	Competitive,
	MannVsMachine,
	Unknown,
}

impl fmt::Display for Gamemode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Gamemode::Casual => write!(f, "Casual"),
			Gamemode::Competitive => write!(f, "Competitive"),
			Gamemode::MannVsMachine => write!(f, "MvM"),
			Gamemode::Unknown => write!(f, "a gamemode"),
		}
	}
}

impl Gamemode {
	pub fn image_key(&self) -> String {
		match self {
			Gamemode::Casual => "casual".to_string(),
			Gamemode::Competitive => "competitive".to_string(),
			Gamemode::MannVsMachine => "mvm".to_string(),
			Gamemode::Unknown => "casual".to_string(),
		}
	}
}

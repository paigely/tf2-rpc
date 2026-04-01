use crate::parser::ParserEvent;
use discord_rich_presence::activity::{Activity, Assets};

pub trait FromParserEvent {
	fn from_parser_event(event: ParserEvent) -> Self;
}

impl<'a> FromParserEvent for Activity<'a> {
	fn from_parser_event(event: ParserEvent) -> Self {
		match event {
			ParserEvent::Connected(info) => Self::new().name("Team Fortress 2").details(info.map),
			ParserEvent::Disconnected => Self::new()
				.name("Team Fortress 2")
				.details("Main Menu")
				.assets(Assets::new().large_image("main_menu").small_image("tf2")),
			ParserEvent::Queuing(gamemode) => {
				let mut activity = Self::new()
					.name("Team Fortress 2")
					.details(format!("Queuing for {gamemode}"));
				if gamemode == "Competitive" {
					activity = activity
						.assets(Assets::new().large_image("competitive").small_image("tf2"));
				}
				activity
			}
		}
	}
}

use crate::{constants::MAP_IMAGES, parser::ParserEvent};
use discord_rich_presence::activity::{Activity, Assets, Party};

pub trait FromParserEvent {
	fn from_parser_event(event: ParserEvent) -> Self;
}

fn map_image(map: String) -> String {
	MAP_IMAGES
		.iter()
		.find(|&&(key, _)| key == map)
		.map(|&(_, value)| value.to_string())
		.unwrap_or_else(|| map)
}

impl<'a> FromParserEvent for Assets<'a> {
	fn from_parser_event(event: ParserEvent) -> Self {
		let base = Assets::new()
			.small_text("tf2-rpc")
			.small_url("https://codeberg.org/paige/tf2-rpc")
			.small_image("tf2mini");
		match event {
			ParserEvent::Connected(info) => base.large_image(map_image(info.map)),
			ParserEvent::Disconnected => base.large_image("main_menu"),
			ParserEvent::Queuing(gamemode) => base.large_image(gamemode.image_key()),
		}
	}
}

impl<'a> FromParserEvent for Activity<'a> {
	fn from_parser_event(event: ParserEvent) -> Self {
		let base = Self::new()
			.name("Team Fortress 2")
			.assets(Assets::from_parser_event(event.clone()));
		match event {
			ParserEvent::Connected(info) => base
				.details(info.map.clone())
				.party(Party::new().size([info.players.into(), info.max_players.into()])),
			ParserEvent::Disconnected => base.details("Main Menu"),
			ParserEvent::Queuing(gamemode) => base.details(format!("Queuing for {gamemode}")),
		}
	}
}

use crate::structs::{Result, Gamemode};
use a2s::{A2SClient, info::Info};
use core::fmt;
use futures::{StreamExt, stream::BoxStream};
use linemux::MuxedLines;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct Parser {
	client: Arc<A2SClient>,
}

#[derive(Debug, Clone)]
pub enum ParserEvent {
	Connected(Info),
	Disconnected,
	Queuing(Gamemode),
}

impl fmt::Display for ParserEvent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParserEvent::Connected(_) => write!(f, "ParserEvent::Connected()"),
			ParserEvent::Disconnected => write!(f, "ParserEvent::Disconnected"),
			ParserEvent::Queuing(msg) => write!(f, "ParserEvent::Queuing({msg})"),
		}
	}
}

impl Parser {
	pub async fn new() -> Result<Self> {
		Ok(Self {
			client: Arc::new(A2SClient::new().await?),
		})
	}

	pub async fn stream_info(&self, path: PathBuf) -> Result<BoxStream<'static, ParserEvent>> {
		let log_path = path.parent().unwrap().join("tf/console.log");
		let mut muxer = MuxedLines::new()?;
		muxer.add_file(&log_path).await?;

		let client = Arc::clone(&self.client);

		let stream = muxer.filter_map(move |event| {
			let client = Arc::clone(&client);
			async move {
				let l = event.ok()?;
				let line = l.line();

				if line.starts_with("Connected to ") {
					let addr_str = line.split("Connected to ").nth(1)?;
					let addr = addr_str.parse::<SocketAddr>().ok()?;
					let info = client.info(addr).await.ok()?;
					return Some(ParserEvent::Connected(info));
				}

				if line.starts_with("Disconnect:") {
					return Some(ParserEvent::Disconnected);
				}

				if line.contains("[PartyClient] Entering queue") {
					if line.ends_with("Ladder Match") {
						return Some(ParserEvent::Queuing(Gamemode::Competitive));
					} else if line.ends_with("Casual Match") {
						return Some(ParserEvent::Queuing(Gamemode::Casual));
						// TODO: i don't know what the string for mann up is
					} else if line.ends_with("MvM Practice") {
						return Some(ParserEvent::Queuing(Gamemode::MannVsMachine));
					} else {
						return Some(ParserEvent::Queuing(Gamemode::Unknown));
					}
				} else if line.starts_with("[PartyClient] Leaving queue") {
					return Some(ParserEvent::Disconnected);
				}

				None
			}
		});

		Ok(stream.boxed())
	}
}

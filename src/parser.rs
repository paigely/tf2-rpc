use crate::structs::{Gamemode, Result};
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

				if let Some(addr_str) = line.strip_prefix("Connected to ") {
					if let Ok(addr) = addr_str.parse::<SocketAddr>() {
						if let Ok(info) = client.info(addr).await {
							return Some(ParserEvent::Connected(info));
						}
					}
					return None;
				}

				if line.starts_with("Disconnect:")
					|| line.starts_with("Sending request to abandon current match")
					|| line.starts_with("Disconnecting from abandoned match server")
					|| line.starts_with("Disconnecting")
				{
					return Some(ParserEvent::Disconnected);
				}

				if line.contains("[PartyClient] Entering queue") {
					let mode = if line.ends_with("Ladder Match") {
						Gamemode::Competitive
					} else if line.ends_with("Casual Match") {
						Gamemode::Casual
					} else if line.ends_with("MvM Practice") || line.ends_with("MannUp") {
						Gamemode::MannVsMachine
					} else {
						Gamemode::Unknown
					};
					return Some(ParserEvent::Queuing(mode));
				} else if line.starts_with("[PartyClient] Leaving queue") {
					return Some(ParserEvent::Disconnected);
				}

				None
			}
		});

		Ok(stream.boxed())
	}
}

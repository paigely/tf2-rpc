mod monitor;
mod parser;
mod structs;
mod util;

use monitor::*;
use parser::*;
use structs::*;
use util::*;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity::Activity};
use futures::StreamExt;
use std::{collections::HashMap, path::PathBuf};
use tokio::task::JoinHandle;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()?
		.block_on(_main())
}

async fn _main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let mut presence = DiscordIpcClient::new("1488729323111190558");
	let mut monitor = Monitor::new(vec![
		// as of the 64-bit update tf2 is no longer named hl2
		// todo: i don't know if this needs an .exe i don't use winblows
		"tf_linux64",
		"tf_win64",
		"tf_win64.exe",
	]);

	let parser = Parser::new().await.unwrap_or_else(|e| {
		tracing::error!("{e}");
		panic!("could not initialise the A2S client's socket")
	});
	let mut active_parsers: HashMap<PathBuf, JoinHandle<()>> = HashMap::new();
	let (tx, mut rx) = tokio::sync::mpsc::channel::<Activity>(32);

	tracing::info!("initialised");

	// todo: i hate how nested and ugly this is, surely there's a cleaner way
	loop {
		tokio::select! {
			Some(activity) = rx.recv() => {
				if let Err(e) = presence.set_activity(activity) {
					tracing::error!("could not set activity: {e}");
				}
			}

			_ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
				for event in monitor.tick() {
					match event {
						MonitorEvent::Started(path) => {
							tracing::info!("process found: {}", path.clone().to_str().unwrap_or(""));

							if let Err(e) = presence.connect() {
								tracing::error!("could not connect to rpc: {e}");
								continue;
							}

							_ = presence.set_activity(Activity::from_parser_event(ParserEvent::Disconnected));

							let tx_clone = tx.clone();
							let parser_clone = parser.clone();
							let path_key = path.clone();

							let handle = tokio::spawn(async move {
								if let Ok(mut info_stream) = parser_clone.stream_info(path).await {
									while let Some(event) = info_stream.next().await {
										_ = tx_clone.send(Activity::from_parser_event(event.clone())).await;
										tracing::info!("{}", event.clone());
									}
								}
							});

							active_parsers.insert(path_key, handle);
						}
						MonitorEvent::Stopped(path) => {
							tracing::info!("process closed: {}", path.clone().to_str().unwrap_or(""));

							_ = presence.clear_activity();
							_ = presence.close();

							if let Some(handle) = active_parsers.remove(&path) {
								handle.abort();
							}
						}
					}
				}
			}

			_ = tokio::signal::ctrl_c() => {
				tracing::info!("shutting down");

				let _ = presence.clear_activity();
				let _ = presence.close();

				for (_, handle) in active_parsers {
					handle.abort();
				}

				return Ok(());
			}
		}
	}
}

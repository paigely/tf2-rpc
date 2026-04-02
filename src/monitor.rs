use std::{collections::HashSet, path::PathBuf};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

pub struct Monitor {
	system: System,
	target_names: Vec<String>,
	active_paths: HashSet<PathBuf>,
}

pub enum MonitorEvent {
	Started(PathBuf),
	Stopped(PathBuf),
}

impl Monitor {
	pub fn new(targets: Vec<&str>) -> Self {
		Self {
			system: System::new_with_specifics(
				RefreshKind::nothing()
					.with_processes(ProcessRefreshKind::nothing().with_exe(UpdateKind::Always)),
			),
			target_names: targets.iter().map(|s| s.to_lowercase()).collect(),
			active_paths: HashSet::new(),
		}
	}

	pub fn tick(&mut self) -> Vec<MonitorEvent> {
		self.system.refresh_processes_specifics(
			ProcessesToUpdate::All,
			true,
			ProcessRefreshKind::nothing().with_exe(UpdateKind::Always),
		);
		let mut events = Vec::new();
		let mut discovered_paths = HashSet::new();

		for target in &self.target_names {
			for process in self.system.processes_by_name(target.as_ref()) {
				if let Some(exe) = process.exe().map(|p| p.to_path_buf()) {
					discovered_paths.insert(exe);
				}
			}
		}

		for path in &discovered_paths {
			if !self.active_paths.contains(path) {
				events.push(MonitorEvent::Started(path.clone()));
			}
		}

		for path in &self.active_paths {
			if !discovered_paths.contains(path) {
				events.push(MonitorEvent::Stopped(path.clone()));
			}
		}

		self.active_paths = discovered_paths;
		events
	}
}

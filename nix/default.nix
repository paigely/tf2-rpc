{
	lib,
	rustPlatform,
}: let
	toml = (lib.importTOML ../Cargo.toml).package;
in
	rustPlatform.buildRustPackage {
		pname = "tf2-rpc";
		inherit (toml) version;

		src =
			lib.fileset.toSource {
				root = ../.;
				fileset =
					lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ../.)) (
						lib.fileset.unions [
							../Cargo.toml
							../Cargo.lock
							../src
						]
					);
			};

		cargoLock.lockFile = ../Cargo.lock;

		meta = {
			license = lib.licenses.gpl3Only;
			maintainers = [];
			mainProgram = "tf2-rpc";
		};
	}

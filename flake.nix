{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

	outputs = {
		self,
		nixpkgs,
		rust-overlay,
		...
	}: let
		inherit (nixpkgs) lib;
		inherit (lib.attrsets) genAttrs;
		inherit (lib.systems) flakeExposed;
		overlays = [(import rust-overlay)];

		forAllSystems = fn:
			genAttrs flakeExposed (
				system:
					fn (
						import nixpkgs {
							inherit system overlays;
							config.allowUnfree = true;
						}
					)
			);
	in {
		packages =
			forAllSystems (pkgs: {
					tf2-rpc = pkgs.callPackage ./nix/default.nix {};
					default = self.packages.${pkgs.stdenv.hostPlatform.system}.tf2-rpc;
				});

		devShells =
			forAllSystems (pkgs: {
					default =
						pkgs.mkShell {
							nativeBuildInputs = with pkgs; [
								pkg-config
							];
							buildInputs = with pkgs; [
								nixd
								alejandra
								(rust-bin.stable.latest.default.override {
										extensions = ["rust-src" "rust-analyzer"];
									})
							];
						};
				});
	};
}

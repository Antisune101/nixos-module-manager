let
	nixpkgs = fetchTarball "https://github.com/Nixos/nixpkgs/tarball/nixos-unstable";
	pkgs = import nixpkgs { config = {}; overlays = []; };
in 
pkgs.mkShell {
	packages = with pkgs; [
		cargo
		rustc
		rust-analyzer
		helix
	];
}

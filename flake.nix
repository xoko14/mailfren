{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        stableToolchain = pkgs.rust-bin.stable.latest.minimal.override {
          extensions = [ "rust-src" "rust-analyzer"];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            stableToolchain
            sqlite
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
          LIBCLANG_PATH = "${pkgs.llvmPackages_11.libclang.lib}/lib";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          DATABASE_URL = "sqlite://./database.db";
        };
      }
    );
}

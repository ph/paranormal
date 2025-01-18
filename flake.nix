{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

	rustVersion = pkgs.rust-bin.nightly.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
            [
              (rustVersion.override { extensions = [ "rust-src" ]; })
              pkgs.rust-analyzer
              pkgs.protobuf
              pkgs.openssl
              pkgs.buf
              pkgs.glibc
              pkgs.cmake
              pkgs.pkg-config
              pkgs.perl
            ];
        };
      });
}

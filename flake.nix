{
  description = "Jasmine Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    # Pinned to the commit the patch in ./patches was generated against.
    rust-analyzer-src = {
      url = "github:rust-lang/rust-analyzer/af4111f0bf85d9d66f4161b007b0081860e0a21c";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      rust-analyzer-src,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # rust-analyzer built from source with fixes for rustc_private projects on
        # read-only (nix) toolchains: lockfile discovery for the dist rustc-src
        # layout, working lockfile copies, and following sysroot dylib symlinks so
        # the prebuilt rustc_macros proc macro loads (gives TyCtxt query getters
        # like `tcx.mir_keys(())` real types).
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
        rust-analyzer-patched = rustPlatform.buildRustPackage {
          pname = "rust-analyzer-patched";
          version = "unstable-2026-08-28";
          src = rust-analyzer-src;
          patches = [ ./patches/rust-analyzer-rustc-private-fixes.patch ];
          cargoLock.lockFile = "${rust-analyzer-src}/Cargo.lock";
          cargoBuildFlags = [
            "--package"
            "rust-analyzer"
            "--bin"
            "rust-analyzer"
          ];
          doCheck = false;
        };

        applications = with pkgs; [
          rust-analyzer-patched
          toolchain

          nixd
          nil
        ];

        libraries = with pkgs; [
          pkg-config
          gcc
        ];
      in
      {
        packages.rust-analyzer = rust-analyzer-patched;

        devShells.default = pkgs.mkShell {
          buildInputs = applications ++ libraries;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" libraries;

          # Make sure the patched rust-analyzer shadows the toolchain's bundled one.
          shellHook = ''
            export PATH=${rust-analyzer-patched}/bin:$PATH
          '';
        };
      }
    );
}

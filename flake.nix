{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
  let
    # system = builtins.currentSystem;
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rustfmt
        rust-analyzer

        # python3
        python312
        maturin
		pypy
		gcc
      cargo-expand
      ];
    };
  };
}

{ pkgs ? import <nixpkgs> {} }:

(pkgs.buildFHSEnv {
    name = "simple-x11-env";
    targetPkgs = pkgs: (with pkgs; [
        python3
        #maturin
		rustc
        cargo
		pypy
		gcc
		rust-analyzer
    ]);
    runScript = "bash";
}).env

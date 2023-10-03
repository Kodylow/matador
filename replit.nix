{ pkgs }: {
  deps = [
        pkgs.lsof
        pkgs.tree
        pkgs.just
        pkgs.cargo
        pkgs.rustc
        pkgs.rust-analyzer
        pkgs.clippy
        pkgs.rustfmt
        pkgs.openssl
        pkgs.pkg-config
  ];
}
{ pkgs }: {
  deps = [
        pkgs.tree
        pkgs.just
        pkgs.pkg-config
        pkgs.openssl
        pkgs.rustup
  ];
}
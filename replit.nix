{ pkgs }: {
  deps = [
        pkgs.tree
        pkgs.pkg-config
        pkgs.openssl
        pkgs.rustup
  ];
}
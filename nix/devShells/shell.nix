{
  mkShell,

  quictests,
  openssl,
}:
mkShell {
  inputsFrom = [ quictests ];

  packages = [
    openssl
  ];
}

{
  mkShell,

  quictests,
}:
mkShell {
  inputsFrom = [ quictests ];

  packages = [
  ];
}

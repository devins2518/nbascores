with import <nixpkgs> { };
let frameworks = pkgs.darwin.apple_sdk.frameworks;
in stdenv.mkDerivation {
  name = "nil-dev";
  buildInputs = [ pkg-config libiconv ]
    ++ lib.optionals stdenv.isDarwin [ frameworks.Security ];
}

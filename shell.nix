with import <nixpkgs> { };
stdenv.mkDerivation {
  name = "nil-dev";
  buildInputs = [ pkg-config openssl ];
}

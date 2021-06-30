with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "nbascores-dev";
    buildInputs = [ pkg-config openssl ];
}

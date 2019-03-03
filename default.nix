{ nixpkgs ? fetchTarball channel:nixos-19.03
, pkgs ? import nixpkgs {}
}:

with pkgs;

rustPlatform.buildRustPackage {
  name = "cardano-http-bridge";

  src = with lib; cleanSourceWith {
    filter = name: type: let baseName = baseNameOf (toString name); in ! (
      (type == "directory" && baseName == "target")
    );
    src = cleanSource ./.;
  };

  buildInputs = [ rustc cargo sqlite protobuf rustfmt ];

  # FIXME: we can remove this once prost is updated.
  PROTOC = "${protobuf}/bin/protoc";

  cargoSha256 = "19g5fy8af65vd9rl66058c67nlrz7r6mjd0cy83865d7q81hdl8r";
}

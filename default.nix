# default.nix
let a=import /etc/nixos/overlays/pkgs.nix; in 
with import <nixpkgs> {
    overlays=[
    a    
    ];
};
stdenv.mkDerivation {
    name = "controller"; # Probably put a more meaningful name here
    buildInputs =  [
    pkg-config
    pgplot
    (stdenv.lib.getLib gfortran.cc)
    xorg.libX11.dev
    xorg.libX11.out
    ];
}

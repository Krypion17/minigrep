{
  description = "Flake for minigrep";

  outputs = { self, nixpkgs }: {

    defaultPackage.x86_64-linux = 
      with import nixpkgs { system = "x86_64-linux"; };
      rustPlatform.buildRustPackage {
        pname = "minigrep";
        version = "1.2.1";

        src = self;

        doCheck = false;

        cargoLock.lockFile = ./Cargo.lock;
      };

  };
}

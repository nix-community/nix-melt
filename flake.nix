{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib)
        genAttrs
        importTOML
        licenses
        maintainers
        sourceByRegex
        ;

      eachSystem = f: genAttrs
        [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-darwin"
          "x86_64-linux"
        ]
        (system: f nixpkgs.legacyPackages.${system});
    in
    {
      formatter = eachSystem (pkgs: pkgs.nixpkgs-fmt);

      herculesCI.ciSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      packages = eachSystem (pkgs:
        let
          src = sourceByRegex self [
            "(src|tests)(/.*)?"
            "Cargo\\.(toml|lock)"
            "build.rs"
          ];

          inherit (pkgs)
            installShellFiles
            rustPlatform
            ;
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "nix-melt";
            inherit ((importTOML (src + "/Cargo.toml")).package) version;

            inherit src;

            cargoLock = {
              lockFile = src + "/Cargo.lock";
            };

            nativeBuildInputs = [
              installShellFiles
            ];

            env = {
              GEN_ARTIFACTS = "artifacts";
            };

            postInstall = ''
              installManPage artifacts/nix-melt.1
              installShellCompletion artifacts/nix-melt.{bash,fish} --zsh artifacts/_nix-melt
            '';

            meta = {
              license = licenses.mpl20;
              maintainers = with maintainers; [ figsoda ];
            };
          };
        });
    };
}

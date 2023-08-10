{
  lib,
  pkgs,
  ...
}: {
  languages = {
    nix.enable = true;
    rust.enable = true;
  };

  packages = [
    pkgs.git

    (lib.mkIf pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.CoreFoundation)
    (lib.mkIf pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.Security)
  ];

  pre-commit = {
    # The Uncompromising Nix Code Formatter.
    hooks.alejandra.enable = true;

    # Check the cargo package for errors.
    # hooks.cargo-check.enable = true;

    # Lint Rust code.
    # hooks.clippy.enable = true;
    # settings.clippy = {
    #   denyWarnings = true;
    #   offline = false;
    # };

    # Check whether the current commit message follows commiting rules.
    # hooks.commitizen.enable = true;

    # Scan Nix files for dead code (unused variable bindings).
    hooks.deadnix.enable = true;
    settings.deadnix.edit = true;

    # Generate a commit message using GPT3.
    # hooks.gptcommit.enable = true;

    # Spell checker and morphological analyzer.
    hooks.hunspell.enable = true;

    # Incremental analysis assistant for writing in Nix.
    hooks.nil.enable = true;

    # Format Rust code.
    hooks.rustfmt.enable = true;

    # Format shell files.
    hooks.shellcheck.enable = true;

    # Lints and suggestions for the Nix programming language.
    hooks.statix.enable = true;

    # Format TOML files with taplo fmt
    hooks.taplo.enable = true;

    # Yaml linter.
    hooks.yamllint.enable = true;
  };
}

{
  lib,
  pkgs,
  ...
}: {
  languages = {
    nix.enable = true;
    rust.enable = true;
  };

  packages =
    [
      pkgs.alejandra
      pkgs.cargo-watch
      pkgs.git
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.mktemp
    ];

  pre-commit.hooks = {
    # The Uncompromising Nix Code Formatter.
    alejandra.enable = true;

    # Check the cargo package for errors.
    # cargo-check.enable = true;

    # Lint Rust code.
    # clippy.enable = true;
    # settings.clippy = {
    #   denyWarnings = true;
    #   offline = false;
    # };

    # Check whether the current commit message follows commiting rules.
    # commitizen.enable = true;

    # Scan Nix files for dead code (unused variable bindings).
    deadnix = {
      enable = true;
      settings.edit = true;
    };

    # Generate a commit message using GPT3.
    # gptcommit.enable = true;

    # Spell checker and morphological analyzer.
    hunspell.enable = true;

    # Incremental analysis assistant for writing in Nix.
    nil.enable = true;

    # Format Rust code.
    rustfmt.enable = true;

    # Format shell files.
    shellcheck.enable = true;

    # Lints and suggestions for the Nix programming language.
    statix.enable = true;

    # Format TOML files with taplo fmt
    taplo.enable = true;

    # Yaml linter.
    yamllint.enable = true;
  };

  scripts.lint-clippy.exec = "${pkgs.cargo}/bin/cargo clippy -- --allow clippy::expect_fun_call --allow clippy::upper_case_acronyms --deny warnings";
}

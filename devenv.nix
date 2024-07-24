{
  lib,
  pkgs,
  ...
}: {
  enterTest = "cargo test";

  languages = {
    nix.enable = true;
    rust.enable = true;
  };

  packages =
    [
      pkgs.alejandra
      pkgs.cargo-watch
      pkgs.git
      pkgs.openssl_3
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.CoreFoundation
      pkgs.darwin.apple_sdk.frameworks.Security
      pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
      pkgs.mktemp
    ];

  pre-commit.hooks = {
    # The Uncompromising Nix Code Formatter.
    alejandra.enable = true;

    # Check the cargo package for errors.
    cargo-check.enable = true;

    # Lint Rust code.
    clippy = {
      enable = true;

      description = "Lint Rust code.";

      # The name of the hook (appears on the report table):
      name = "clippy";

      # The command to execute (mandatory):
      package = pkgs.cargo;
      entry = "cargo clippy -- --allow clippy::expect_fun_call --allow clippy::upper_case_acronyms --deny warnings";

      # The pattern of files to run on (default: "" (all))
      # see also https://pre-commit.com/#hooks-files
      files = "(\\.rs|^devenv\\.nix)$";

      # List of file types to run on (default: [ "file" ] (all files))
      # see also https://pre-commit.com/#filtering-files-with-types
      # You probably only need to specify one of `files` or `types`:
      # types = ["file"];

      # Exclude files that were matched by these patterns (default: [ ] (none)):
      # excludes = [];

      # The language of the hook - tells pre-commit
      # how to install the hook (default: "system")
      # see also https://pre-commit.com/#supported-languages
      # language = "system";

      # Set this to false to not pass the changed files
      # to the command (default: true):
      pass_filenames = false;
    };

    # Check whether the current commit message follows commiting rules.
    commitizen.enable = true;

    # Scan Nix files for dead code (unused variable bindings).
    deadnix = {
      enable = true;
      settings.edit = true;
    };

    # Incremental analysis assistant for writing in Nix.
    nil.enable = true;

    # Opinionated multi-language code formatter.
    prettier.enable = true;

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
    yamllint.settings.configData = "{ rules: { comments: { min-spaces-from-content: 1 } } }";
  };
}

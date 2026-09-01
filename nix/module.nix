self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.repo-builder;
in
{
  options.programs."kestrel" = {
    enable = lib.mkEnableOption "Monitoring TUI app for divers infos.";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "kestrel.packages.\${system}.default";
      description = "Monitoring TUI app for divers infos.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}

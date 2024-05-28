# thanks getchoo :3
self: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.ukubot-rs;

  inherit (pkgs.stdenv.hostPlatform) system;

  inherit
    (lib)
    getExe
    literalExpression
    mdDoc
    mkDefault
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    types
    ;
in {
  options.services.ukubot-rs = {
    enable = mkEnableOption "ukubot-rs";
    package = mkPackageOption self.packages.${system} "ukubot-rs" {};
    environmentFile = mkOption {
      description = mdDoc ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = types.nullOr types.path;
      default = null;
      example = literalExpression ''
        "/run/agenix.d/1/ukubot-rs"
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services."ukubot-rs" = {
      enable = true;
      wantedBy = mkDefault ["multi-user.target"];
      wants = mkDefault ["network-online.target"];
      after = mkDefault ["network.target" "network-online.target"];
      script = ''
        ${getExe cfg.package}
      '';

      serviceConfig = {
        Type = "simple";
        Restart = "always";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        # hardening
        DynamicUser = true;
        PrivateTmp = true;
        NoNewPrivileges = true;
        RestrictNamespaces = "uts ipc pid user cgroup";
        ProtectSystem = "strict";
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        PrivateDevices = true;
        RestrictSUIDSGID = true;
      };
    };
  };
}

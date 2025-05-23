# thanks getchoo :3
self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.ukubot-rs;

  inherit (pkgs.stdenv.hostPlatform) system;

  inherit (lib)
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
in
{
  options.services.ukubot-rs = {
    enable = mkEnableOption "ukubot-rs";
    package = mkPackageOption self.packages.${system} "default" { };
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
    services.redis.servers.ukubot = {
      enable = true;
      user = "ukubot";
      port = 0; # disable tcp
    };

    systemd.services."ukubot-rs" = {
      enable = true;
      wantedBy = mkDefault [ "multi-user.target" ];
      wants = mkDefault [ "network-online.target" ];
      after = mkDefault [
        "network.target"
        "network-online.target"
        "redis-ukubot.service"
      ];
      script = ''
        ${getExe cfg.package}
      '';

      environment = {
        REDIS_URL = "unix:${config.services.redis.servers.ukubot.unixSocket}";
      };

      serviceConfig = {
        Type = "simple";
        Restart = "always";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        User = "ukubot";
        Group = "ukubot";

        # hardening
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

    users = {
      groups.ukubot = { };
      users.ukubot = {
        isSystemUser = true;
        group = "ukubot";
      };
    };
  };
}

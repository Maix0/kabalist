{
  kabalist-web,
  kabalist-server,
}: {
  pkgs,
  config,
  lib,
  ...
}:
with lib; {
  options.services.kabalist = {
    enable = mkEnableOption "kabalist, a shared list manager";

    package = mkOption {
      type = types.package;
      default = kabalist-server;
    };

    secret = mkOption {
      type = types.str;
    };

    port = mkOption {
      type = types.port;
      default = 8080;
    };

    user = mkOption {
      type = types.str;
      default = "kabalist";
    };

    enableFrontend = mkEnableOption "kabalist web application";
  };

  config = let
    cfg = config.services.kabalist;
  in
    mkIf cfg.enable {
      systemd.services.kabalist = {
        description = "kabalist";
        after = ["network.target" "postgresql.service"];
        wantedBy = ["multi-user.target"];

        serviceConfig = {
          Type = "simple";
          User = cfg.user;
          Group = "kabalist";
          ExecStart = "${cfg.package}/bin/kabalist_api";
          # Security
          NoNewPrivileges = true;
          # Sandboxing
          ProtectSystem = "strict";
          ProtectHome = true;
          PrivateTmp = true;
          PrivateDevices = true;
          PrivateUsers = true;
          ProtectHostname = true;
          ProtectClock = true;
          ProtectKernelTunables = true;
          ProtectKernelModules = true;
          ProtectKernelLogs = true;
          ProtectControlGroups = true;
          RestrictAddressFamilies = ["AF_UNIX AF_INET AF_INET6"];
          LockPersonality = true;
          MemoryDenyWriteExecute = true;
          RestrictRealtime = true;
          RestrictSUIDSGID = true;
          PrivateMounts = true;
        };

        environment =
          {
            KABALIST_JWT_SECRET = cfg.secret;
            KABALIST_PORT = toString cfg.port;
            KABALIST_DATABASE_URL = "postgres://${cfg.user}/kabalist?host=/var/run/postgresql";
            KABALIST_TEMPLATES = "${cfg.package}/share";
          }
          // lib.optionalAttrs cfg.enableFrontend {
            KABALIST_FRONTEND = "${kabalist-web}";
          };
      };

      services.postgresql = {
        ensureUsers = [
          {
            name = cfg.user;
            ensurePermissions = {"DATABASE \"kabalist\"" = "ALL PRIVILEGES";};
          }
        ];
        ensureDatabases = ["kabalist"];
      };

      users.users = mkIf (cfg.user == "kabalist") {
        kabalist = {
          description = "Kabalist Service";
          group = "kabalist";
          isSystemUser = true;
        };
      };
      users.groups.kabalist = {};
    };
}
{
  lib,
  beamPackages,
  overrides ? (x: y: {}),
}: let
  buildRebar3 = lib.makeOverridable beamPackages.buildRebar3;
  buildMix = lib.makeOverridable beamPackages.buildMix;
  buildErlangMk = lib.makeOverridable beamPackages.buildErlangMk;

  self = packages // (overrides self packages);

  packages = with beamPackages;
  with self; {
    earmark_parser = buildMix rec {
      name = "earmark_parser";
      version = "1.4.39";

      src = fetchHex {
        pkg = "earmark_parser";
        version = "${version}";
        sha256 = "06553a88d1f1846da9ef066b87b57c6f605552cfbe40d20bd8d59cc6bde41944";
      };

      beamDeps = [];
    };

    ex_doc = buildMix rec {
      name = "ex_doc";
      version = "0.34.0";

      src = fetchHex {
        pkg = "ex_doc";
        version = "${version}";
        sha256 = "60734fb4c1353f270c3286df4a0d51e65a2c1d9fba66af3940847cc65a8066d7";
      };

      beamDeps = [earmark_parser makeup_elixir makeup_erlang];
    };

    makeup = buildMix rec {
      name = "makeup";
      version = "1.1.2";

      src = fetchHex {
        pkg = "makeup";
        version = "${version}";
        sha256 = "cce1566b81fbcbd21eca8ffe808f33b221f9eee2cbc7a1706fc3da9ff18e6cac";
      };

      beamDeps = [nimble_parsec];
    };

    makeup_elixir = buildMix rec {
      name = "makeup_elixir";
      version = "0.16.2";

      src = fetchHex {
        pkg = "makeup_elixir";
        version = "${version}";
        sha256 = "41193978704763f6bbe6cc2758b84909e62984c7752b3784bd3c218bb341706b";
      };

      beamDeps = [makeup nimble_parsec];
    };

    makeup_erlang = buildMix rec {
      name = "makeup_erlang";
      version = "1.0.0";

      src = fetchHex {
        pkg = "makeup_erlang";
        version = "${version}";
        sha256 = "ea7a9307de9d1548d2a72d299058d1fd2339e3d398560a0e46c27dab4891e4d2";
      };

      beamDeps = [makeup];
    };

    nimble_parsec = buildMix rec {
      name = "nimble_parsec";
      version = "1.4.0";

      src = fetchHex {
        pkg = "nimble_parsec";
        version = "${version}";
        sha256 = "9c565862810fb383e9838c1dd2d7d2c437b3d13b267414ba6af33e50d2d1cf28";
      };

      beamDeps = [];
    };
  };
in
  self

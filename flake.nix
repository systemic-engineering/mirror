{
  description = "mirror — the sub-Turing compiler. Grammar all the way down.";

  outputs = { self, ... }: {
    # The 8KB binary. Built by mirror craft.
    # Until mirror self-hosts: ~/.local/bin/mirror (the bootstrap)
    packages.aarch64-darwin.default = self.packages.aarch64-darwin.mirror;
    packages.aarch64-darwin.mirror = builtins.path {
      name = "mirror";
      path = ./bin;
    };
  };
}

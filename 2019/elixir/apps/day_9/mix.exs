defmodule Day9.MixProject do
  use Mix.Project

  def project do
    [
      app: :day_9,
      version: "0.1.0",
      build_path: "../../_build",
      config_path: "../../config/config.exs",
      deps_path: "../../deps",
      lockfile: "../../mix.lock",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  defp deps do
    [
      {:intcode, in_umbrella: true},
      {:util, in_umbrella: true}
    ]
  end
end

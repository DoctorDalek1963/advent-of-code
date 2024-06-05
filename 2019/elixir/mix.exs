defmodule Aoc2019.MixProject do
  use Mix.Project

  def project do
    [
      apps_path: "apps",
      name: "aoc_2019",
      version: "0.1.0",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Dependencies listed here are available only for this
  # project and cannot be accessed from applications inside
  # the apps folder.
  defp deps do
    [
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end
end

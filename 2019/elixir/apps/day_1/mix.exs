defmodule Day1.MixProject do
  use Mix.Project

  def project do
    [
      app: :day_1,
      version: "0.1.0",
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  defp deps do
    []
  end
end

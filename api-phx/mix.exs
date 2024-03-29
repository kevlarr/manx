defmodule Manx.MixProject do
  use Mix.Project

  def project, do: [
    app: :manx,
    version: "0.1.0",
    elixir: "~> 1.5",
    elixirc_paths: elixirc_paths(Mix.env()),
    compilers: [:phoenix, :gettext] ++ Mix.compilers(),
    start_permanent: Mix.env() == :prod,
    aliases: aliases(),
    deps: deps()
  ]

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application, do: [
    mod: {Manx.Application, []},
    extra_applications: [:logger, :runtime_tools]
  ]

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps, do: [
    {:phoenix, "~> 1.4.0"},
    {:phoenix_pubsub, "~> 1.1"},
    {:phoenix_ecto, "~> 4.0"},
    {:ecto_sql, "~> 3.0"},
    {:postgrex, ">= 0.0.0"},
    {:phoenix_html, "~> 2.11"},
    {:phoenix_live_reload, "~> 1.2", only: :dev},
    {:gettext, "~> 0.11"},
    {:jason, "~> 1.0"},
    {:plug_cowboy, "~> 2.0"},
    {:comeonin, "~> 4.1.2"},
    {:argon2_elixir, "~> 1.2.2"},
  ]

  defp aliases, do: [
    "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
    "ecto.reset": ["ecto.drop", "ecto.setup"],
    "ecto.redo": ["ecto.rollback", "ecto.migrate"],
    test: ["ecto.create --quiet", "ecto.migrate", "test"]
  ]
end

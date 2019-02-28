defmodule Manx.Repo do
  use Ecto.Repo,
    otp_app: :manx,
    adapter: Ecto.Adapters.Postgres
end

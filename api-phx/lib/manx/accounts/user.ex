defmodule Manx.Accounts.User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "users" do
    field :email, :string
    field :password, :string, virtual: true
    field :password_hash, :string

    timestamps()
  end

  def changeset(user, attrs) do
    user
    |> cast(attrs, [:email, :password])
    |> validate_required([:email, :password])
    |> validate_format(:email, ~r/@/)
    |> validate_length(:password, min: 12, max: 60)
    |> unique_constraint(:email)
    |> hash_password()
  end

  defp hash_password(chset) do
    case chset do
      %Ecto.Changeset{valid?: true, changes: %{password: pw}} ->
        put_change(chset, :password_hash, Comeonin.Argon2.hashpwsalt(pw))
      _ ->
        chset
    end
  end
end

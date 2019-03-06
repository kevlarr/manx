defmodule Manx.Accounts.User do
  use ManxWeb, :model

  schema "users" do
    has_many :organization_users, Manx.Orgs.OrganizationUser
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

  defp hash_password(changeset) do
    case changeset do
      %Ecto.Changeset{valid?: true, changes: %{password: pw}} ->
        put_change(changeset, :password_hash, Comeonin.Argon2.hashpwsalt(pw))
      _ ->
        changeset
    end
  end
end

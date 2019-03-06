defmodule Manx.Orgs.OrganizationUser do
  use ManxWeb, :model

  schema "organization_users" do
    belongs_to :user, Manx.Accounts.User
    belongs_to :organization, Manx.Orgs.Organization
    field :name, :string
    field :username, :string
    timestamps()
  end

  @doc """
  Base changeset that allows updating user-editable fields.
  """
  def changeset(org_user, attrs) do
    org_user
    |> cast(attrs, [:name, :username])
    |> validate_required([:name, :username])
  end

  @doc """
  Changeset for new organization registration. Does not validate
  uniqueness on org/user or org/username, but those cannot fail
  when creating org. user for a new organization.
  """
  def registration_changeset(user, org, attrs) do
    user
    |> build_assoc(:organization_users)
    |> changeset(attrs)
    |> put_change(:organization, org)
  end
end

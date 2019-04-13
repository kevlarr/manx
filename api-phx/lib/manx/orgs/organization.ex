defmodule Manx.Orgs.Organization do
  use ManxWeb, :model
  import Ecto.Query, only: [from: 2]

  alias __MODULE__
  alias Manx.Accounts.User
  alias Manx.Orgs
  alias Manx.Orgs.OrganizationUser

  schema "organizations" do
    belongs_to :creator, User
    has_many :organization_users, OrganizationUser
    field :title, :string
    field :short_id, :string
    timestamps()
  end

  @doc """
  Queries all Organization connected to a given user
  """
  def for_user(%Manx.Accounts.User{id: user_id}) do
    from org in Organization,
      join: org_user in OrganizationUser,
      on: org_user.organization_id == org.id,
      where: org_user.user_id == ^user_id,
      select: [org, org_user]
  end

  @doc """
  Base changeset that allows updating user-editable fields.
  """
  def changeset(%Organization{} = org, attrs \\ %{}) do
    org
    |> cast(attrs, [:title])
    |> validate_required([:title])
  end

  @doc """
  Changeset for a new organization, which will assign a random
  short base64 ID to use as a user-facing unique key.
  """
  def registration_changeset(user, attrs) do
    %Organization{}
    |> changeset(attrs)
    |> put_change(:creator, user)
    |> Orgs.assign_short_id()
  end
end

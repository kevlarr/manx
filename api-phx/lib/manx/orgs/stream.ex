defmodule Manx.Orgs.Stream do
  use ManxWeb, :model

  import Ecto.Query, only: [from: 2]

  alias __MODULE__
  alias Manx.Accounts.User
  alias Manx.Orgs
  alias Manx.Orgs.Organization
  alias Manx.Orgs.OrganizationUser
  alias Manx.Orgs.StreamUser

  schema "streams" do
    belongs_to :creator, User
    belongs_to :organization, Organization
    belongs_to :parent, Stream
    field :global, :boolean
    field :name, :string
    field :short_id, :string
    timestamps()
  end

  @doc """
  Loads all streams for the given org. users
  """
  def for_org_users(org_users) do
    org_user_ids = for org_user <- org_users, do: org_user.id

    from stream in Stream,
      join: stream_user in StreamUser,
      on: stream_user.stream_id == stream.id,
      join: org_user in OrganizationUser,
      on: stream_user.organization_user_id == org_user.id,
      where: org_user.id in ^org_user_ids,
      select: stream
  end

  @doc """
  Base changeset that allows updating user-editable fields.
  """
  def changeset(%Stream{} = stream, attrs \\ %{}) do
    stream
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end

  @doc """
  Sets up the base, global stream for an organization upon org. creation
  """
  def registration_changeset(%User{} = creator, %Organization{} = org, attrs) do
    %Stream{}
    |> changeset(attrs)
    |> put_change(:creator, creator)
    |> put_change(:organization, org)
    |> put_change(:global, true)
    |> Orgs.assign_short_id()
  end
end

defmodule Manx.Orgs.Organization do
  use ManxWeb, :model
  import Ecto.Query, only: [from: 2]

  alias __MODULE__
  alias Manx.Accounts.User
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
      join: orguser in OrganizationUser,
      on: orguser.organization_id == org.id,
      where: orguser.user_id == ^user_id,
      select: org
  end

  @doc """
  Base changeset that allows updating user-editable fields.
  """
  def changeset(org, attrs \\ %{}) do
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
    |> assign_short_id()
  end

  @doc """
  Adds a newly-created random short URL-safe ID to the changeset
  """
  defp assign_short_id(changeset) do
    case changeset do
      %Ecto.Changeset{valid?: true} ->
        put_change(changeset, :short_id, generate_short_id())
      _ ->
        changeset
    end
  end

  @doc """
  Genereates a random, 6-char URL-safe ID
  """
  defp generate_short_id() do
    :crypto.strong_rand_bytes(6)
    |> Base.url_encode64
    |> binary_part(0, 6)
  end
end

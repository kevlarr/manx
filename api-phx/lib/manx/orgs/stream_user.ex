defmodule Manx.Orgs.StreamUser do
  use ManxWeb, :model

  alias __MODULE__
  alias Manx.Orgs.OrganizationUser
  alias Manx.Orgs.Stream

  schema "stream_users" do
    belongs_to :organization_user, OrganizationUser
    belongs_to :stream, Stream
    timestamps()
  end

  @doc """
  Builds a new stream user for the given stream and org user
  """
  def registration_changeset(%OrganizationUser{} = org_user, %Stream{} = stream) do
    %StreamUser{}
    |> cast(%{}, [])
    |> put_change(:organization_user, org_user)
    |> put_change(:stream, stream)
  end
end

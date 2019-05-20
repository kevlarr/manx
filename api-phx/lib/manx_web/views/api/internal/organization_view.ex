defmodule ManxWeb.Api.Internal.OrganizationView do
  use ManxWeb, :view

  alias Manx.Orgs.Organization
  alias Manx.Orgs.OrganizationUser
  alias Manx.Orgs.Stream

  @doc """
  Serializes each list of records into JSON-compatible maps
  """
  def render("index.json", %{orgs: orgs, org_users: org_users, streams: streams}) do
    # FIXME: use render_many
    %{
      organizations: (for org <- orgs, do: org_json(org)),
      organizationUsers: (for org_user <- org_users, do: org_user_json(org_user)),
      streams: (for stream <- streams, do: stream_json(stream)),
    }
  end

  @doc """
  Accepts an organization and an orguser and serializes into a map
  """
  def render("create.json", %{org: org, org_user: org_user, stream: stream}), do: %{
    organization: org_json(org),
    organizationUser: org_user_json(org_user),
    stream: stream_json(stream),
  }

  @doc """
  Serializes an Organization model
  """
  def org_json(%Organization{} = org), do: %{
    id: org.id,
    creatorId: org.creator_id,
    title: org.title,
    shortId: org.short_id,
  }

  @doc """
  Serializes an OrganizationUser model
  """
  def org_user_json(%OrganizationUser{} = org_user), do: %{
    id: org_user.id,
    organizationId: org_user.organization_id,
    name: org_user.name,
  }

  @doc """
  Serializes a Stream model
  """
  def stream_json(%Stream{} = stream), do: %{
    id: stream.id,
    organizationId: stream.organization_id,
    parentId: stream.parent_id,
    global: stream.global,
    name: stream.name,
    shortId: stream.short_id,
  }
end

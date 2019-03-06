defmodule ManxWeb.Api.Internal.OrganizationView do
  use ManxWeb, :view

  alias Manx.Orgs.Organization
  alias Manx.Orgs.OrganizationUser

  def render("create.json", %{org: org, org_user: org_user}), do: %{
    organization: org_json(org),
    organization_user: org_user_json(org_user),
  }

  def org_json(%Organization{title: title, short_id: short_id}), do: %{
    title: title,
    short_id: short_id,
  }

  def org_user_json(%OrganizationUser{name: name, username: username}), do: %{
    name: name,
    username: username,
  }
end

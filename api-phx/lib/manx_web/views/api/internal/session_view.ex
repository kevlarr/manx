defmodule ManxWeb.Api.Internal.SessionView do
  use ManxWeb, :view

  import ManxWeb.Api.Internal.OrganizationView, only: [org_json: 1]

  def render("create.json", %{orgs: orgs}), do: %{
    organizations: (for org <- orgs, do: org_json(org))
  }
end

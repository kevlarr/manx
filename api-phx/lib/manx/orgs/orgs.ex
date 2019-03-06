defmodule Manx.Orgs do
  @moduledoc """
  Context for organizations.
  """

  alias Ecto.Multi
  alias Manx.Orgs.Organization
  alias Manx.Orgs.OrganizationUser
  alias Manx.Repo

  #def get_organization_by_short(short_id) do
    #Repo.get_by(Organization, %{short_id: short_id})
  #end

  @doc """
  Creates a new organization and org. user with the given sets
  of attributes, linking to the provided existing user.
  """
  def create_organization(user, org_attrs, org_user_attrs) do
    org_changeset = Organization.registration_changeset(user, org_attrs)

    Multi.new()
    |> Multi.insert(:org, org_changeset)
    |> Multi.run(:org_user, fn repo, %{org: org} ->
      user
      |> OrganizationUser.registration_changeset(org, org_user_attrs)
      |> repo.insert()
    end)
    |> Repo.transaction()
  end
end

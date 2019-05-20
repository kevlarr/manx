defmodule Manx.Orgs do
  @moduledoc """
  Context for organizations.
  """

  import Ecto.Changeset, only: [put_change: 3]

  alias Ecto.Multi
  alias Manx.Orgs.Organization
  alias Manx.Orgs.OrganizationUser
  alias Manx.Orgs.Stream
  alias Manx.Orgs.StreamUser
  alias Manx.Repo

  @doc """
  Creates a new organization and org. user with the given sets
  of attributes, linking to the provided existing user, and creates
  the default global stream
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
    |> Multi.run(:stream, fn repo, %{org: org, org_user: org_user} ->
      org_user
      |> Stream.registration_changeset(org, %{name: org.title})
      |> repo.insert()
    end)
    |> Multi.run(:stream_user, fn repo, %{org_user: org_user, stream: stream} ->
      org_user
      |> StreamUser.registration_changeset(stream)
      |> repo.insert()
    end)
    |> Repo.transaction()
  end

  @doc """
  Adds a newly-created random short URL-safe ID to the changeset
  """
  def assign_short_id(changeset) do
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
  def generate_short_id() do
    :crypto.strong_rand_bytes(6)
    |> Base.url_encode64
    |> binary_part(0, 6)
  end
end

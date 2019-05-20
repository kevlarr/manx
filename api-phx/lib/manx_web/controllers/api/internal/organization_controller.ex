defmodule ManxWeb.Api.Internal.OrganizationController do
  use ManxWeb, :controller

  plug :ensure_authenticated when action in [:index, :create]

  @doc """
  Finds all organizations accessible to current user (along with corresponding
  orguser) and serializes them as response
  """
  def index(conn, _params) do
    orgs_with_user =
      conn.assigns.current_user
      |> Manx.Orgs.Organization.for_user()
      |> Manx.Repo.all()

    {orgs, org_users} = Enum.unzip for l <- orgs_with_user, do: List.to_tuple(l)

    streams =
      org_users
      |> Manx.Orgs.Stream.for_org_users()
      |> Manx.Repo.all()

    conn
    |> put_status(200)
    |> render("index.json",
      orgs: orgs,
      org_users: org_users,
      streams: streams
    )
  end

  @doc """
  Accepts params for a new organization and orguser, creates models, and
  returns results
  """
  def create(conn, %{"organization" => org_params, "organizationUser" => org_user_params}) do
    user = conn.assigns.current_user

    case Manx.Orgs.create_organization(user, org_params, org_user_params) do
      {:ok, result} ->
        conn
        |> put_status(200)
        |> render("create.json",
          org: result.org,
          org_user: result.org_user,
          stream: result.stream
        )

      {:error, _, changeset, _} ->
        conn
        |> put_status(422)
        |> put_view(ErrView)
        |> render(:"422", changeset: changeset)
    end
  end
end

defmodule ManxWeb.Api.Internal.OrganizationController do
  use ManxWeb, :controller

  plug :ensure_authenticated when action in [:index, :create]

  def index(conn, _params) do
    orgs =
      conn.assigns.current_user
      |> Manx.Orgs.Organization.for_user()
      |> Manx.Repo.all()

    conn
    |> put_status(200)
    |> render("index.json", orgs: orgs)
  end

  def create(conn, %{"organization" => org_params, "organization_user" => org_user_params}) do
    user = conn.assigns.current_user

    case Manx.Orgs.create_organization(user, org_params, org_user_params) do
      {:ok, result} ->
        conn
        |> put_status(201)
        |> render("create.json", org: result.org, org_user: result.org_user)

      {:error, _, changeset, _} ->
        conn
        |> put_status(422)
        |> put_view(ErrView)
        |> render(:"422", changeset: changeset)
    end
  end
end

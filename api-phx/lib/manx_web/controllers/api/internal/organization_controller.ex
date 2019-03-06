defmodule ManxWeb.Api.Internal.OrganizationController do
  use ManxWeb, :controller

  plug :authorize when action in [:create]

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

  # FIXME - this should be a common plug, not copy/paste
  defp authorize(conn, _opts) do
    if conn.assigns.current_user do
      conn
    else
      conn
      |> halt()
      |> put_status(403)
      |> put_view(ErrView)
      |> render(:"403")
    end
  end
end

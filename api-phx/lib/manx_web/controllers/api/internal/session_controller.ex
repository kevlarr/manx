defmodule ManxWeb.Api.Internal.SessionController do
  use ManxWeb, :controller

  def create(conn, %{"session" => %{"email" => email, "password" => pw}}) do
    case Manx.Auth.authenticate(conn, email, pw) do
      {:ok, conn} ->
        orgs =
          conn.assigns.current_user
          |> Manx.Orgs.Organization.for_user()
          |> Manx.Repo.all()

        conn
        |> put_status(200)
        |> render("create.json", orgs: orgs)

      {:error, conn} ->
        conn
        |> put_status(401)
        |> put_view(ErrView)
        |> render(:"401")
    end
  end

  def delete(conn, _params) do
    conn
    |> Manx.Auth.sign_out()
    |> send_resp(204, "")
  end
end

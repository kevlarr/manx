defmodule ManxWeb.Api.Internal.SessionController do
  use ManxWeb, :controller

  def create(conn, %{"session" => %{"email" => email, "password" => pw}}) do
    case Manx.Auth.authenticate(conn, email, pw) do
      {:ok, conn} ->
        conn
        |> send_resp(201, "")

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

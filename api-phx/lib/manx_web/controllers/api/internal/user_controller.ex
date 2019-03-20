defmodule ManxWeb.Api.Internal.UserController do
  use ManxWeb, :controller

  alias Manx.Accounts

  plug :ensure_authenticated when action in [:delete]

  def create(conn, %{"user" => user_params}) do
    case Accounts.create_user(user_params) do
      {:ok, user} ->
        conn
        |> Manx.Auth.sign_in(user)
        |> put_status(200)
        |> json(%{})

      {:error, %Ecto.Changeset{} = changeset} ->
        conn
        |> put_status(422)
        |> put_view(ErrView)
        |> render(:"422", changeset: changeset)
    end
  end

  def delete(conn, %{"id" => user_id}) do
    # FIXME - check current user ID matches
    conn
    |> put_status(200)
    |> json(%{hi: user_id})
  end
end

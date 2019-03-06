defmodule Manx.Auth do
  import Plug.Conn

  alias Manx.Accounts

  def init(opts), do: opts

  def call(conn, _opts) do
    user_id = get_session(conn, :user_id)
    user = user_id && Accounts.get_user(user_id)
    assign(conn, :current_user, user)
  end

  def authenticate(conn, email, password) do
    user = Accounts.get_user_by(email: email)

    cond do
      user && Accounts.authenticate(user, password) ->
        {:ok, conn |> sign_in(user)}
      user ->
        {:error, conn}
      true ->
        # If no user, then do a dummy check to help mitigate timing attacks
        Comeonin.Argon2.dummy_checkpw()
        {:error, conn}
    end
  end

  def sign_in(conn, user), do: conn
    |> assign(:current_user, user)
    |> put_session(:user_id, user.id)
    |> configure_session(renew: true)

  def sign_out(conn), do: conn
    |> configure_session(drop: true)
end

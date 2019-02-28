defmodule Manx.Accounts do
  @moduledoc """
  The Context for user accounts.
  """

  alias Manx.Repo
  alias Manx.Accounts.User

  def get_user(id), do: Repo.get(User, id)

  def get_user!(id), do: Repo.get!(User, id)

  def get_user_by(params), do: Repo.get_by(User, params)

  #def change_user(%User{} = user) do
    #User.changeset(user, %{})
    #end
  #

  def authenticate(%User{password_hash: hash}, password) do
    Comeonin.Argon2.checkpw(password, hash)
  end

  def create_user(attrs \\ %{}) do
    %User{}
    |> User.changeset(attrs)
    |> Repo.insert()
  end
end

defmodule Manx.Accounts do
  @moduledoc """
  Context for user accounts.
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

  @doc """
  Checks the provided password against the hash stored for a user.
  """
  def authenticate(%User{password_hash: hash}, password) do
    Comeonin.Argon2.checkpw(password, hash)
  end

  @doc """
  Creates a new user with the given attributes via the base changeset
  and attempts to insert into the Repo.
  """
  def create_user(attrs \\ %{}) do
    %User{}
    |> User.changeset(attrs)
    |> Repo.insert()
  end
end

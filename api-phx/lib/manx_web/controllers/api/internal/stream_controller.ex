defmodule ManxWeb.Api.Internal.StreamController do
  use ManxWeb, :controller

  plug :ensure_authenticated when action in [:update]

  def update(conn, _params) do
  end
end

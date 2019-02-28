defmodule ManxWeb.PageController do
  use ManxWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end

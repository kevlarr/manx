defmodule ManxWeb.Api.Internal.TopicController do
  use ManxWeb, :controller

  plug :ensure_authenticated when action in [:create]


  def create(conn, %{"stream_id" => stream_id, "topic" => topic_params}) do
    IO.inspect(topic_params)
    # TODO
    conn
    |> put_status(200)
    |> render("create.json", topic: topic_params)
  end
end

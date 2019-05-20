defmodule ManxWeb.Api.Internal.TopicView do
  use ManxWeb, :view

  def render("create.json", %{topic: topic}), do: %{
    topic: topic,
  }
end

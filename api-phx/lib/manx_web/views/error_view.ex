defmodule ManxWeb.ErrorView do
  use ManxWeb, :view

  def render("422.json", %{changeset: changeset}), do: %{
    errors: Ecto.Changeset.traverse_errors(changeset, &translate_error/1),
  }

  def template_not_found(template, _assigns) do
    Phoenix.Controller.status_message_from_template(template)
  end
end

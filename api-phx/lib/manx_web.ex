defmodule ManxWeb do
  @moduledoc """
  The entrypoint for defining your web interface, such
  as controllers, views, channels and so on.

  This can be used in your application as:

      use ManxWeb, :controller
      use ManxWeb, :view

  The definitions below will be executed for every view,
  controller, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below. Instead, define any helper function in modules
  and import those modules here.
  """

  def model do
    quote do
      use Ecto.Schema

      import Ecto
      import Ecto.Changeset
    end
  end

  def controller do
    quote do
      use Phoenix.Controller, namespace: ManxWeb

      import Plug.Conn
      import ManxWeb.Gettext
      alias ManxWeb.Router.Helpers, as: Routes
      alias ManxWeb.ErrorView, as: ErrView
    end
  end

  def view do
    quote do
      use Phoenix.View,
        root: "lib/manx_web/templates",
        namespace: ManxWeb

      # Import convenience functions from controllers
      import Phoenix.Controller, only: [get_flash: 1, get_flash: 2, view_module: 1]

      # Use all HTML functionality (forms, tags, etc)
      use Phoenix.HTML

      import ManxWeb.ErrorHelpers
      import ManxWeb.Gettext
      alias ManxWeb.Router.Helpers, as: Routes
    end
  end

  def router do
    quote do
      use Phoenix.Router
      import Plug.Conn
      import Phoenix.Controller
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
      import ManxWeb.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end

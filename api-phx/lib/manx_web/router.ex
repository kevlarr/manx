defmodule ManxWeb.Router do
  use ManxWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
    plug :fetch_session
    plug Manx.Auth
  end

  scope "/api", ManxWeb.Api do
    pipe_through :api

    scope "/internal", Internal do
      resources "/organizations", OrganizationController, only: [:index, :create, :delete] do
        resources "/streams", StreamController, only: [:index, :create]
      end
      resources "/sessions", SessionController, only: [:create, :delete]
      resources "/streams", StreamController, only: [:update] do
          resources "/topics", TopicController, only: [:create]
      end
      resources "/users", UserController, only: [:create, :delete]
    end
  end
end

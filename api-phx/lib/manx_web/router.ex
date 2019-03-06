defmodule ManxWeb.Router do
  use ManxWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug Manx.Auth
  end

  pipeline :api do
    plug :accepts, ["json"]
    plug :fetch_session
    plug Manx.Auth
  end

  scope "/", ManxWeb do
    pipe_through :browser

    get "/", PageController, :index
  end

  scope "/api", ManxWeb.Api do
    pipe_through :api

    scope "/internal", Internal do
      resources "/organizations", OrganizationController, only: [:create, :delete]
      resources "/sessions", SessionController, only: [:create, :delete]
      resources "/users", UserController, only: [:create, :delete]
    end
  end
end

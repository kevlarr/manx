Rails.application.routes.draw do
  namespace "api", defaults: { format: "json" } do
    namespace "internal" do
      post :sessions, to: "sessions#create"
      post :users, to: "users#create"
    end
  end
end

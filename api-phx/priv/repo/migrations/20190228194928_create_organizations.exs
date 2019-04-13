defmodule Manx.Repo.Migrations.CreateOrganizations do
  use Ecto.Migration

  def change do
    create table(:organizations) do
      # Do not delete organization if the creator is
      add(:creator_id, references(:users, on_delete: :nilify_all))

      add(:title, :string, null: false)
      add(:short_id, :string, null: false)
      timestamps()
    end

    create table(:organization_users) do
      # Delete all org. users if org. itself is deleted
      add(:organization_id, references(:organizations, on_delete: :delete_all), null: false)

      # Do not delete org. user if parent user is deleted - leave around for record of name
      add(:user_id, references(:users, on_delete: :nilify_all))

      add(:name, :string, null: false)
      add(:username, :string, null: false)
      timestamps()
    end

    create unique_index(:organizations, [:short_id])
    create unique_index(:organization_users, [:organization_id, :user_id])
    create unique_index(:organization_users, [:organization_id, :username])
  end
end

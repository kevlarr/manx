defmodule Manx.Repo.Migrations.CreateStreams do
  use Ecto.Migration

  def change do
    create table(:streams) do
      # Deleting the parent user should NOT remove the record
      add(:creator_id, references(:users, on_delete: :nilify_all))

      # Deleting a parent stream (if present) should just make this top-level
      add(:parent_id, references(:streams, on_delete: :nilify_all))

      # Deleting the organization should clear all the things
      add(:organization_id, references(:organizations, on_delete: :delete_all), null: false)

      add(:global, :boolean, default: false)
      add(:name, :string, null: false)
      add(:short_id, :string, null: false)
      timestamps()
    end

    create table(:stream_users) do
      # Delete the stream user if the stream itself goes away
      add(:stream_id, references(:streams, on_delete: :delete_all), null: false)

      # Don't take an action on delete - org. users should not be deleted
      add(:organization_user_id, references(:organization_users), null: false)

      timestamps()
    end

    create unique_index(:streams, [:organization_id, :short_id])
    create unique_index(:streams, [:organization_id], where: :global)
    create unique_index(:stream_users, [:stream_id, :organization_user_id])
  end
end

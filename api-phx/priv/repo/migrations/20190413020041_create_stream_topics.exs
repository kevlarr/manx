defmodule Manx.Repo.Migrations.CreateStreamTopics do
  use Ecto.Migration

  def change do
    create table(:stream_topics) do
      add(:creator_id, references(:stream_users), null: false)
      add(:stream_id, references(:streams, on_delete: :delete_all), null: false)
      add(:raw, :string, null: false)
      add(:rendered, :string, null: false)
      timestamps()
    end
  end
end

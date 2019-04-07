defmodule Manx.Repo.Migrations.RemoveOrgUserUsername do
  use Ecto.Migration

  def up do
    drop index(:organization_users, [:organization_id, :username])
    alter table(:organization_users) do
      remove(:username)
    end
  end

  def down do
    alter table(:organization_users) do
      add(:username, :string)
    end
    create unique_index(:organization_users, [:organization_id, :username])
  end
end

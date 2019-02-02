class CreateUsers < ActiveRecord::Migration[5.2]
  def change
    create_table :users do |t|
        t.string :email, null: false, limit: 256
        t.text :password_digest, null: false
        t.timestamps
    end
    add_index :users, :email, unique: true
  end
end

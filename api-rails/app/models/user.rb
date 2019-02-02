class User < ApplicationRecord
  has_secure_password
  has_secure_token

  validates :email,
    presence: true,
    uniqueness: true,
    length: { maximum: 256 },
    format: /\A([^\s]+)((?:[-a-z0-9]\.)[a-z]{2,})\z/i

  validates :password,
    length: { minimum: 12 },
    allow_nil: true
end

require 'rails_helper'

RSpec.describe User, type: :model do

  context "validations" do

    describe "@email" do

      it "must be a valid format" do
        password = "password12345"

        u = build :user, email: "kevin", password: password
        expect(u).not_to be_valid

        errs = u.errors.messages
        expect(errs[:email]).to eq ["is invalid"]

        u = build :user, email: "kevin@some", password: password
        expect(u).not_to be_valid

        u = build :user, email: "kevin@some.where", password: password
        expect(u).to be_valid
      end

      it "must be unique" do
        email = "k.e+vin@some.where"
        password = "password12345"

        u1 = create :user, email: email, password: password
        expect(u1).to be_valid

        u2 = build :user, email: email, password: password
        expect(u2).not_to be_valid
        expect(u2.errors.messages[:email]).to eq ["has already been taken"]
      end

      it "must be 256 chars or less" do
        password = "password123456"
        email = "#{"a" * 250}@a.com"

        u1 = build :user, email: email, password: password
        u2 = build :user, email: email + "a", password: password

        expect(u1).to be_valid
        expect(u2).not_to be_valid

      end
    end

    describe "@password" do

      it "must be the right length" do
        email = "kevin@some.where"

        u0 = build :user, email: email
        u1 = build :user, email: email, password: "12345678901"
        u2 = build :user, email: email, password: "123456789012"

        expect(u0).not_to be_valid
        expect(u1).not_to be_valid
        expect(u2).to be_valid
      end

      it "lets user be loaded and saved" do
        create :user, email: "hey@you.guys", password: "11223344556677"
        user = User.find_by email: "hey@you.guys"

        expect(user).to be_valid
      end
    end
  end
end

require "rails_helper"

RSpec.describe "Internal::Users API" do

  describe "POST /api/internal/users" do

    context "valid params" do
      let(:params) {{
        email: "kevin@some.where",
        password: "password1234",
      }}

      before { post "/api/internal/users", params: params }

      it "returns 200" do
        expect(response.status).to eq(201)
      end

      it "returns no body" do
        expect(response.body).to eq("")
      end

      it "creates a user" do
        user = User.find_by(email: "kevin@some.where")
        expect(user).not_to be_nil
      end
    end

    context "duplicate email" do
      let(:params) {{
        email: "kevin@some.where",
        password: "password123456",
      }}

      before do
        create :user, email: "kevin@some.where", password: "asdfasdfasdf"
        post "/api/internal/users", params: params
      end

      it "returns 400" do
        expect(response.status).to eq(400)
      end

      it "includes message about email" do
        body = JSON.parse response.body
        expect(body["email"]).to eq(["has already been taken"])
      end

      it "does not create a user" do
        expect(User.all.count).to eq(1)
      end
    end

    context "invalid email" do
      let(:params) {{
        email: "kevin@where",
        password: "password1234",
      }}

      before { post "/api/internal/users", params: params }

      it "returns 400" do
        expect(response.status).to eq(400)
      end

      it "includes message about email" do
        body = JSON.parse(response.body)
        expect(body["email"]).to eq(["is invalid"])
      end

      it "does not create a user" do
        expect(User.all.count).to eq(0)
      end
    end

    context "invalid password" do
      let(:params) {{
        email: "kevin@some.where",
        password: "pass",
      }}

      before { post "/api/internal/users", params: params }

      it "returns 400" do
        expect(response.status).to(eq(400))
      end

      it "includes message about password" do
        body = JSON.parse(response.body)
        expect(body["password"]).to eq(["is too short (minimum is 12 characters)"])
      end

      it "does not create a user" do
        expect(User.all.count).to eq(0)
      end
    end

    context "no params" do
      before { post "/api/internal/users" }

      it "returns 400" do
        expect(response.status).to(eq(400))
      end

    end
  end
end

require "rails_helper"

RSpec.describe "Internal::Sessions API" do
  let!(:user) { create :user,
    email: "kevin@some.where",
    password: "password1234"
  }

  describe "POST /api/internal/sessions" do

    context "valid params" do
      let(:params) {{
        email: "kevin@some.where",
        password: "password1234",
      }}

      before { post "/api/internal/sessions", params: params }

      it "returns 200" do
        expect(response.status).to eq(201)
      end

    end

    context "invalid email" do
      let(:params) {{
        email: "kevin@where",
        password: "password1234",
      }}

      before { post "/api/internal/sessions", params: params }

      it "returns 400" do
        expect(response.status).to eq(400)
      end
    end

    context "invalid password" do
      let(:params) {{
        email: "kevin@some.where",
        password: "pass",
      }}

      before { post "/api/internal/sessions", params: params }

      it "returns 400" do
        expect(response.status).to(eq(400))
      end
    end

    context "no params" do
      before { post "/api/internal/sessions" }

      it "returns 400" do
        expect(response.status).to(eq(400))
      end

    end
  end
end

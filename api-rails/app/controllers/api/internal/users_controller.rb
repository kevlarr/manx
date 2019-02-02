module Api
  module Internal
    class UsersController < ApplicationController
      def create
        user = User.new(
          email: params[:email],
          password: params[:password],
        )

        if user.valid? && user.save
          head :created
        else
          render json: user.errors.messages, status: :bad_request
        end
      end
    end
  end
end

module Api
  module Internal
    class SessionsController < ApplicationController
      def create
        render json: {hi: :there}
      end
    end
  end
end

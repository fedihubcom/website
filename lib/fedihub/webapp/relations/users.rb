# frozen_string_literal: true

module Fedihub
  module Webapp
    module Relations
      class Users < ROM::Relation[:sql]
        schema infer: true
      end
    end
  end
end

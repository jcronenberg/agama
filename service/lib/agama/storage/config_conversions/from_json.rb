# frozen_string_literal: true

# Copyright (c) [2024] SUSE LLC
#
# All Rights Reserved.
#
# This program is free software; you can redistribute it and/or modify it
# under the terms of version 2 of the GNU General Public License as published
# by the Free Software Foundation.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, contact SUSE LLC.
#
# To contact SUSE LLC about this file by physical or electronic mail, you may
# find current contact information at www.suse.com.

require "agama/config"
require "agama/storage/config_builder"
require "agama/storage/config_conversions/from_json_conversions/config"

module Agama
  module Storage
    module ConfigConversions
      # Config conversion from JSON hash according to schema.
      class FromJSON
        # @param config_json [Hash]
        # @param product_config [Agama::Config, nil]
        def initialize(config_json, product_config: nil)
          # TODO: Replace product_config param by a ProductDefinition.
          @config_json = config_json
          @product_config = product_config || Agama::Config.new
        end

        # Performs the conversion from Hash according to the JSON schema.
        #
        # @return [Storage::Config]
        def convert
          # TODO: Raise error if config_json does not match the JSON schema.
          FromJSONConversions::Config
            .new(config_json, config_builder: config_builder)
            .convert
        end

      private

        # @return [Hash]
        attr_reader :config_json

        # @return [Agama::Config]
        attr_reader :product_config

        # @return [ConfigBuilder]
        def config_builder
          @config_builder ||= ConfigBuilder.new(product_config)
        end
      end
    end
  end
end

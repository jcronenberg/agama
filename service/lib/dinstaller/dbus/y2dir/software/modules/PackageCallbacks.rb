# Copyright (c) [2022] SUSE LLC
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

require "yast"
require "logger"
require "dinstaller/software/callbacks"
require "dinstaller/dbus/clients/questions_manager"

# :nodoc:
module Yast
  # Replacement for the Yast::PackageCallbacks module.
  class PackageCallbacksClass < Module
    def main
      puts "Loading mocked module #{__FILE__}"
    end

    # @see https://github.com/yast/yast-yast2/blob/19180445ab935a25edd4ae0243aa7a3bcd09c9de/library/packages/src/modules/PackageCallbacks.rb#L183
    def InitPackageCallbacks(logger = ::Logger.new($stdout))
      DInstaller::Software::Callbacks::Signature.new(
        questions_manager, logger
      ).setup

      DInstaller::Software::Callbacks::Media.new(
        questions_manager, logger
      ).setup
    end

    def questions_manager
      @questions_manager ||= DInstaller::DBus::Clients::QuestionsManager.new 
    end
  end

  PackageCallbacks = PackageCallbacksClass.new
  PackageCallbacks.main
end

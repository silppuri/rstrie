require 'helix_runtime'

begin
  require 'rstrie/native'
rescue LoadError
  warn "Unable to load rstrie/native. Please run `rake build`"
end

class Rstrie
  def add(key, value = -1)
    insert(key, value)
  end
end

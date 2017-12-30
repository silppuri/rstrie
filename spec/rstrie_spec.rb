require 'spec_helper'

RSpec.describe Rstrie do
  describe '.new' do
    it "returns a Rstrie instance" do
      expect(Rstrie.new).is_a? Rstrie
    end
  end

  describe '.insert' do
    it "echoes" do
      Rstrie.insert('asd')
    end
  end
end

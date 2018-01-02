require 'spec_helper'

RSpec.describe Rstrie do
  before :each do
    @trie = Rstrie.new;
    @trie.add('rocket')
    @trie.add('rock')
    @trie.add('frederico')
  end

  describe :has_key? do
    it 'returns true for words in the trie' do
      expect(@trie.has_key?('rocket')).to eq true
    end

    it 'returns false for words that are not in the trie' do
      expect(@trie.has_key?('not_in_the_trie')).to eq false
    end
  end

  describe :get do
    it 'returns -1 for words in the trie without a weight' do
      expect(@trie.get('rocket')).to eq(-1)
    end

    it 'returns nil if the word is not in the trie' do
      expect(@trie.get('not_in_the_trie')).to eq nil
    end
  end

  describe :add do
    it 'adds a word to the trie' do
      expect(@trie.add('forsooth')).to eq true
      expect(@trie.get('forsooth')).to eq(-1)
    end

    it 'adds a word with a weight to the trie' do
      expect(@trie.add('chicka',123)).to eq true
      expect(@trie.get('chicka')).to eq 123
    end

    it 'adds values greater than 16-bit allows' do
      expect(@trie.add('chicka', 72_000)).to eq true
      expect(@trie.get('chicka')).to eq 72_000
    end

    it 'adds a word with a non-numeric value to the trie' do
      expect(@trie.add('doot', 'Heeey')).to eq true
      expect(@trie.get('doot')).to eq 'Heeey'
    end
  end

  describe :delete do
    it "deletes a word from the trie" do
      @trie.delete('rocket')
      expect(@trie.has_key?('rocket')).to eq false
    end
  end

  describe :children do
    it 'returns all words beginning with a given prefix' do
      children = @trie.children('roc')
      expect(children.size).to eq 2
      expect(children).to include('rock')
      expect(children).to include('rocket')
    end

    it 'returns blank array if prefix does not exist' do
      expect(@trie.children('ajsodij')).to be_empty
    end

    it 'includes the prefix if the prefix is a word' do
      children = @trie.children('rock')
      expect(children.size).to eq 2
      expect(children).to include('rock')
      expect(children).to include('rocket')
    end

    it 'returns blank array if prefix is nil' do
      expect(@trie.children(nil)).to be_empty
    end
  end
end

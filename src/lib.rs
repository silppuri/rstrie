#[macro_use]
extern crate helix;
extern crate sequence_trie;

use sequence_trie::SequenceTrie;

ruby! {
    pub class Rstrie {
        struct {
            trie: SequenceTrie<String,i64>
        }

        def initialize(helix) {
            Rstrie { helix, trie: SequenceTrie::<String, i64>::new() }
        }
        def insert(&mut self, key: String, value: i64) -> bool {
            self.trie.insert(&[key], value);
            true
        }
        def get(&self, key: String) -> Option<i64> {
           match self.trie.get(&[key]) {
               None => None,
               Some(i) => Some(*i)
           }
        }
    }
}

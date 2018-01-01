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
            match self.trie.insert(&[key], value) {
                None => true,
                Some(_) => false
            }
        }
        def get(&self, key: String) -> Option<i64> {
           match self.trie.get(&[key]) {
               None => None,
               Some(i) => Some(*i)
           }
        }

        def has_key(&self, key: String) -> bool {
           match self.trie.get(&[key]) {
               None => false,
               Some(_) => true
           }
        }

        def delete(&mut self, key: String) {
            self.trie.remove(&[key])
        }
    }
}

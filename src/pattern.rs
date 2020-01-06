pub struct SinglePattern {
    pub seq: &'static [u8],
    pub table: &'static [usize],
}

impl SinglePattern {
    pub fn match_against(&self, haystack: &[u8]) -> Option<usize> {
        let mut hay_idx = 0usize;
        let mut pat_idx = 0usize;
        while hay_idx < haystack.len() {
            if self.seq[pat_idx] == haystack[hay_idx] {
                pat_idx += 1;
                hay_idx += 1;
            };

            if pat_idx == self.seq.len() {
                return Some(hay_idx - pat_idx);
            };

            if hay_idx < haystack.len() && self.seq[pat_idx] != haystack[hay_idx] {
                if pat_idx != 0 {
                    pat_idx = self.table[pat_idx - 1];
                } else {
                    hay_idx += 1;
                };
            };
        };

        None
    }
}

pub trait ITrieNode<V: 'static + Copy> {
    fn get_value(&self) -> Option<V>;
    fn get_child(&self, c: u8) -> Option<&dyn ITrieNode<V>>;
}

pub struct TrieLeafNode<V: 'static + Copy> {
    pub value: Option<V>,
}

impl<V: 'static + Copy> ITrieNode<V> for TrieLeafNode<V> {
    fn get_value(&self) -> Option<V> {
        self.value
    }

    fn get_child(&self, c: u8) -> Option<&dyn ITrieNode<V>> {
        None
    }
}

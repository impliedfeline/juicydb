use std::fs::File;
use crate::db::*;

/// B-tree datatype, consisting of a file handle and an in-memory root node. B-trees can be seen as
/// an on-disk data structure for tables. Each table in juicydb is stored in it's own file and
/// b-tree nodes in these files are broken up into contiguous 4kb pages. When reading/updating
/// database values, these pages are loaded into memory and flushed to disk as needed.
///
/// Conceptually, a b-tree consists of a set of `k` keys and `k + 1` children. The keys partition
/// the range of keys in the children; in binary-trees, the left child consists of keys less than
/// the one in the current node, and the right child consists of keys greater than the one in the
/// current node. A b-tree is simply a generalization of this scheme to multiple keys, where the
/// child "in between two keys" has keys greater than or equal to the key left of the child, but
/// less than the key right of the child. In juicydb, k == 255, meaning that each node has 255
/// keys and 256 children. This property of splitting to multiple children, referred to as fanout,
/// is typically high in b-trees to reduce the height of built trees. Smaller height means that we
/// need fewer "jumps" in the tree to locate a key and thus fewer disk seeks, which are relatively
/// expensive.
///
/// Each file begins with a (4kb) header node, consisting of e.g. schema information. The exact format
/// for headers is under construction. The header is followed by 1 or more b-tree nodes. For the
/// file format of b-tree nodes, refer to the documentation on [`BTreeNode`]s.

pub struct BTree {
    file: File,
    root: BTreeNode,
}

/// A B-tree node datatype. A node is either internal to the tree, or a leaf node which represents
/// a row in the database. The page format in juicydb is referred to as slotted pages; this means
/// that (after the header) each page consists of a contiguous segment of keys pointing to
/// fixed-size segments in the same page. These segments are referred to as cells. The cells have a
/// key and in the case of internal nodes, a page id, giving the offset to a page of a child, and
/// in the case of leaf nodes, a data record i.e. a database row. The keys in the beginning of a
/// page are sorted according to the key contained in the cell they are pointing to; this means we
/// can perform a binary search on the pointers for fast access of children in the b-tree.
///
/// As each node (page) has at most 256 children (cells), the keys can be represented as 8-bit
/// unsigned integers. Keys and page ID's are both represented as unsigned 32-bit integers, meaning
/// that a database table may have at most 2(cells), the keys can be represented as 8-bit unsigned
/// integers. Keys and page ID's are both represented as unsigned 32-bit integers, meaning that a
/// table can hold at most 2^32 = 4294967296 rows, and the file representing a table can have a
/// maximum file size of 4kb * 2^32 ~= 16 terabytes.
pub enum BTreeNode {
    Internal { cells: Cell<Key,PageId> },
    Leaf { cells: Cell<Key, Row> },
}

type Key = u32;
type PageId = u32;

/// An in-memory datastructure representing a cell in a page. Essentially an AVL-tree.
pub struct Cell<K, V> {
    key: K,
    value: V,
    left: Option<Box<Cell<K,V>>>,
    right: Option<Box<Cell<K,V>>>,
}

impl<K, V> Cell<K,V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value, left: None, right: None }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq + PartialOrd,
    {
        if &self.key == key {
            Some(&self.value)
        } else if &self.key > key {
            self.left.as_ref().and_then(|left| left.get(key))
        } else {
            self.right.as_ref().and_then(|right| right.get(key))
        }
    }

    // TODO: implement balancing
    pub fn insert(&mut self, key: K, value: V)
    where
        K: PartialEq + PartialOrd,
    {
        if self.key == key {
            self.value = value;
        } else if self.key > key {
            if let Some(left) = self.left.as_mut() {
                left.insert(key, value)
            } else {
                self.left = Some(Box::new(Self::new(key, value)));
            }
        } else {
            if let Some(right) = self.right.as_mut() {
                right.insert(key, value)
            } else {
                self.right = Some(Box::new(Self::new(key, value)));
            }
        }
    }

}


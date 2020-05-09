use crate::db::*;
use std::fs::File;

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
    schema: Schema,
}

pub struct KeyCell {
    pub key: u32,
    pub page_id: u32,
}

pub enum BTreeNode {
    Internal {
        freecells: [bool; 256],
        pointers: [u8; 256],
        cells: [KeyCell; 256],
    },
    Leaf {
        freecells: [bool; 64],
        pointers: [u8; 64],
        data_cells: [Row; 64],
    },
}

impl BTreeNode {
    pub fn read(input: [u8; 4096]) -> Self {
        match input[0] {
            b'0' => {
                let freecells = {
                    let mut bool_array = [false; 256];
                    for (i, byte) in input[1..257].iter().enumerate() {
                        match byte {
                            b'0' => bool_array[i] = false,
                            b'1' => bool_array[i] = true,
                            _ => panic!("Invalid freecell list"),
                        }
                    }
                    bool_array
                };
                let pointers = {
                    let mut byte_array = [b'0'; 256];
                    for (i, byte) in input[1792..2048].iter().enumerate() {
                        byte_array[i] = *byte;
                    }
                    byte_array
                };
                let cells = {
                    let mut cell_array = [KeyCell { key: 0, page_id: 0 }; 256];
                    for i in 0..256 {
                        let mut key_bytes = [b'0', b'0', b'0', b'0'];
                        for (i, byte) in input[(i * 8 + 2048)..(i * 8 + 2052)].iter().enumerate() {
                            key_bytes[i] = *byte;
                        }
                        let mut page_id_bytes = [b'0', b'0', b'0', b'0'];
                        for (i, byte) in input[(i * 8 + 2052)..(i * 8 + 2056)].iter().enumerate() {
                            page_id_bytes[i] = *byte;
                        }
                        let key = ((key_bytes[0] as u32) << 24)
                            | ((key_bytes[1] as u32) << 16)
                            | ((key_bytes[2] as u32) << 8)
                            | ((key_bytes[3] as u32) << 0);
                        let page_id = ((page_id_bytes[0] as u32) << 24)
                            | ((page_id_bytes[1] as u32) << 16)
                            | ((page_id_bytes[2] as u32) << 8)
                            | ((page_id_bytes[3] as u32) << 0);
                        cell_array[i] = KeyCell { key, page_id };
                    }
                    cell_array
                };
                BTreeNode::Internal {
                    freecells,
                    pointers,
                    cells,
                }
            }
            b'1' => {
                let freecells = {
                };
                let pointers = {
                };
                let data_cells = {
                };
            }
            _ => panic!("Invalid enum flag"),
        }
    }
}

impl BTree {
    /*
        pub fn serialize(&self) {
            let header_page: [u8; 4096] = {
                let schema_text = &self
                    .schema
                    .iter()
                    .map(|(column_name, db_type)| format!("{} {}", column_name, db_type))
                    .collect::<Vec<String>>()
                    .join(", ");

                let mut page = [0; 4096];
                for (i, byte) in schema_text.bytes().enumerate().take(4096) {
                    page[i] = byte;
                }
                page
            };
        }
    */
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
/*
pub enum BTreeNode {
    Internal { cells: Cell<Key, PageId> },
    Leaf { cells: Cell<Key, Row> },
}
*/

type Key = u32;
type PageId = u32;

/*
/// An in-memory datastructure representing a cell in a page. Essentially an AVL-tree.
pub struct Cell<K, V> {
    key: K,
    value: V,
    left: Option<Box<Cell<K, V>>>,
    right: Option<Box<Cell<K, V>>>,
}

impl<K, V> Cell<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
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

    pub fn remove(&mut self, key: K)
    where
        K: PartialEq + PartialOrd,
    {
        todo!();
    }
}
*/

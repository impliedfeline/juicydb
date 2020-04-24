# Implementation notes

The program consists of a command-line parser, a query processor and a storage
manager

## Command-line parser - `parser.rs`

The parser consists of data type definitions encoding the ASTs of SQL queries
and command-line metacommands, as well as the parser for these constructs. The
parser itself is implemented as an iterator over an input string, producing
AST-nodes corresponding to the commands. The parsers rely heavily on
`Result`-types, which are a kind of enum (disjoint/tagged union to be precise)
representing either success or failure. The errors are encoded in their own type
`ParseError`.

## Query processor - to be implemented

## Storage manager - `btree.rs`

Handles the loading and writing of data on disk. Database tables are stored in
files in a b-tree format. For the specifics of the file format, refer to the
documentation in `btree.rs` 


juicydb is a simple relational database. In particular, it will support:
- (atleast a subset of) SQL
- [indexing](https://en.wikipedia.org/wiki/Database_index)
- (hopefully) a rudimentary query planner/optimizer

Features that are out of scope for this project and thus will NOT be supported
include:
- access control
- concurrent queries/multi-user environments
- ACID transactions

Internally, tables and indexes will be represented using
[B-trees](https://en.wikipedia.org/wiki/B-tree). B-trees support the following
operations with the following time complexities:
- search: `O(log n)`
- insert: `O(log n)`
- delete: `O(log n)`
The use of B-trees in representing data in a database is a well-known technique
and implemented in several production databases, including
[PostgreSQL](https://www.postgresql.org/docs/9.2/indexes-types.html) and the
[Oracle
Database](https://docs.oracle.com/cd/E11882_01/server.112/e40540/indexiot.htm#CNCPT721).
Support for other indexing schemes may be implemented as time allows.

Each row in a table has a unique identifier, referred to as the primary key.
The primary key acts as an index for the data in a table, meaning that the data
in a table may be sorted and arranged in a B-tree. Thus queries that concern a
single primary key will have the above time complexities.

Querying over non-indexed data will always have a worst case time complexity of
`O(n)`. To speed up queries over columns other than the primary key, the tables
may be indexed by a column. Indexing by a column constructs a B-tree that has as
it's values records consisting of the given column and a pointer to the row in
the original table. The data in the B-tree will be sorted based on the column,
making the time complexities of the queries follow the above table. Note that
when indexing over columns with non-unique values, the above time complexities
may in general fail.

juicydb will accept SQL queries over the command-line and (potentially) over a
UNIX socket. The full list of supported queries will be determined at a later
date. In addition juicydb will (hopefully) support certain query optimizations
such as join ordering.

As sources to guide the implementation, I will be referring to the following
literature (where appropriate):
- [Database Management Systems](http://pages.cs.wisc.edu/~dbbook/)
- [Architecture of a Database
  System](https://dsf.berkeley.edu/papers/fntdb07-architecture.pdf)
- [Readings in Database Systems](http://www.redbook.io/)

the website:
- [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/)

and the documentation and implementations of:
- SQLite
- PostgreSQL


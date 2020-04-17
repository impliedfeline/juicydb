What did I do:
- 9.4. Implement command line parser (8 hours)
- 17.4. Sketch out b-tree implementation, document it and implement slotted page
  cells as binary trees. (4 hours)
- 17.4. Test and document parser (4 hours)
How did the development progress;
- Fairly well, plenty to do still. There are certain kinks with the on-disk
  b-tree specification that need to be ironed out
What did I learn:
- B-tree implementation techniques, slotted pages, etc. Most of these points are
  elaborated on in the documentation at [`btree.rs`](src/btree.rs)
Problems:
- I'm late with the project so I have some catching up to do; in general though
  I know everything I need to do going forward. Biggest problem with the project
  is that I still don't have a MVP ready.
What's next:
- Get the project to MVP status; at this point I think the best strategy is to
  implement the data structure first and move on from there, especially since
  interfacing with files and specific file formats (tables as pages) is hard to
  decouple from certain parts of program logic, mainly storage management. After
  that, joins need to be implemented.

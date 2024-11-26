# completeRustTutorial

## Two ways to create a rust project

- one is end user application. command is `cargo init`
- second is library `cargo init --lib`

![rust_project_create](./rust_project_create.png)

## println is a macro. It is not a function.

## Enums

- There are some enums Rust provides you by default specifically the Option and the Result enum

## Package mangement

- in Rust external library is called crate. So to add a new crate to the project enter `cargo add chrono` to the
  terminal.

## Memory allocation

![memory_allocation](./memory_allocation.png)

![memory_mangement](./memory_mangement.png)

![memory_mangement2](./memory_mangement2.png)

![memory_mangement3](./memory_mangement3.png)

## Ownership rules

- Each value in Rust has a owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped.

![ownership](./ownership.png)

## Rules of reference

- At a given time you can have either one mutable reference or any number of immutable references.

## Iterators

- Iterators allows you to perform some task on a sequence of items. An iterator is responsible for the logic of
  iterating over each item and determine when the sequence is finished. In Rust Iterators are lazy they don't have any
  effect until you call the methods that consume the iterator. `v1.iter()` has no effect until we consume it.

- Under the hood for loop uses iterator. It is just that we don't see those complexities.

![for_loop_iterator](./for_loop_iterator.png)

![different_iterators](./different_iterators.png)

the for syntax when directly applied on the collection uses into_iter under the hood.

![for_loop_iter](./for_loop_iter.png)

## consuming adapters

- Methods that call next are called consuming adaptors, because calling them uses up the iterator.

## Iterator adapters

- Iterator adapters are methods defined on the iterator trait that don't consume the iterator, Instead they produce
  different iterators by changing some aspect of the original iterator.

## Strings and slices

- Strings and slices both are UTF-8 encoded.
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. String
  slices let you reference a contiguous sequence of elements in a string. So slices is a kind of generic concept, they
  apply to vectors, strings etc. A slice is a kind of a reference so it doesn't have ownership

![strings_stack_heap](./strings_stack_heap.png)

## Macros
- println! , vec! all are macros. These get expanded to bigger code.
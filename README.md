# Buffers (proof of concept)
This is a new representation on how to conceptialize how a vector (and other containers) manage their data.

Currently it is expected that the data is saved in a continuous slice of memory from the heap. The addition of Allocators means that you may control how this slice of memory is managed, but the underlying assumption keeps being the same.

In this project I add an abstraction in between the container (vector) and the actual memory, which splits the responsabilities in different interfaces:

 - Allocator (or `std::alloc`): Its responsible to manage slices of memory on the heap.
 - Buffer: Its responsability is to read and write values to memory, and aquire and release memory (if it can) but doesn't track what values have been writen, or removed.
 - Container (Vector): Is responsible to manage where the values are

At first this abstraction seems a bit unnecessary, but the original need for this split came from trying to make a struct of arrays (SoA) by only managing memory ([see soa_derive's issue](https://github.com/lumol-org/soa-derive/issues/19)). This case is different because the data is actually split into multiple slices of memory (which is the whole point of a SoA), so the stadard aproach doesn't work for it. Once this case was covered I discovered the the new abstraction became composable and could compose optimizations to a buffer really simply. One such optimization is an Small Vector Optimization (SVO) which would be really hard to add into the standard (which, in fact, doesn't). It also make it possible to use buffers on the stack with `InlineBuffer`.

Most of the working have been heavily inspired by the standard [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), `RawVec` (internal only), and [the Rustonomicon's `RawVec`](https://doc.rust-lang.org/nomicon/vec/vec-raw.html).

## How using it looks like
Actually you can just import `Vector` and use the default configuration:
```rust
use generic_vec::Vector;

let vector: Vector<usize> = Vector::new();
```

This project will try to make it as similar as `Vec` but a lot of work is still missing, so only a few methods are actually implemented.


You can also create your own buffer stack. For example an inline buffer:
```rust
use generic_vec::Vector;
use buffers::inline::InlineBuffer;

type StackBuffer = InlineBuffer<usize, 200>; // Has a 200 elements limit but it's on the stack
let vector: Vector<usize, StackBuffer> = Vector::new();
```

## How it's structured
This project is forcefully ordered. That means that I manually numerated the module files and are usually sorted from more important to less. Modules starting with a letter can be thought as apendixes and are usually utilities.

### Vector
Has a single file with the current vector methods.

### Buffers
Bufferes are seperated in 3 parts:
  1. `interface`: Where the traits and its documentation live.
  1. `base_buffers`: Where the basic buffers (leaf of the composite) are.
  1. `composites`: Buffers that compose with others. All the optimizations (SVO and ZSTO) are here.

## To be done
  - [ ] Better naming (help please)
  - [ ] Add the most commonly used methods
  - [ ] Upload to Cargo
  - [ ] Create prelude (?)
  - [ ] Add optimizations (mainly forcing/avoiding some inlines and adding some hints)
  - [ ] Achieve parity with std

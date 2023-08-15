⚠ While I belive that this to be correct, it needs improvement. Expect changes
after I recieve feedback

# Buffers
Another way of looking at memory management for collections.

To define a buffer, compose the parts you'd like, and then use it in a
collection:
```rust
use buffers::{
  base_buffers::HeapBuffer,
  composites::{
    ZstoBuffer,
    ExponentialGrowthBuffer,
    SvoBuffer,
  },
};

use buffers_collections::vec::Vector;

type ExampleBuffer<T> = ZstoBuffer<  // Optimize if T is a Zero-Sized Type
  ExponentialGrowthBuffer< // Make the buffer to grow exponentially (powers of 2)
    SvoBuffer<          // Add Small Vector Optimization
      128,              // Size of the small vector (in number of elements)
      HeapBuffer<T>,    // Save the values on the heap (base buffer)
    >
  >
>;

let mut example_vector: Vector<u32, ExampleBuffer<_>> = Vector::new();
```

## The model
Currently when using collections, they are responsible for managing its memory.
If you want a different layout than the provided, you must reimplement the
entire collection. A good example of this (an the one started it all) is
[SoA derive](https://github.com/lumol-org/soa-derive) which has to reimplement
`Vec` and all its functions, a lot of times by simply copying the source code
with some type annotation change to simply make a vector with a structure of
arrays. This is also visible in the standard library: there is an undocumented
`RawVec`, which has a lot of common functions for memory management and was
created to reduce duplication.

If you squint a bit, you may see that what's common in this cases is that the
collections have multiple responsibilities:
  1. Saving and retrieving objects and values with some properties.
  2. Managing the memory necessary to do so.

Multiple collections may use the same underlying layout (eg. `Vec` and
`VecDeque`), so what would happen if we made an abstraction for just it? This
is the result.

I called this abstraction a **buffer** and it turns out it has some nice
properties (some which I didn't notice at first):
  1. Reuse in multiple collections (so one specification can be reused)
  1. Remove the knowledge on how the memory is obtained
  1. Allows dynamic and static memory versions (static vector for free)
  1. Allows non-contiguous memory layouts (like SoA)
  1. Isolation and composition of optimizations, and thus allows for targeted
  optimizations for each use case when necessary (see composition)
  1. Simpler code


## Difference from an allocator
An allocator is responsible for the necessary strategies to share the entire or
part of the system memory when requested. It supports dynamic layout and is used
by multiple collections (and buffers).

A buffer may use one or multiple allocators (see `HeapBuffer` and
`AllocatorBuffer`) and use them to aquire memory. But it doesn't care about how
it's saved on the actual memory, nor the relationship needs to be one to one.

## Buffer as a composite
The usual implementation of collections have some optimizations. This
optimizations are balanced to be good enough for most use cases. If a use case
is specific enough, then you use another collection.

As it turns out, you can make a buffer which uses another buffer underneath.
This composite structure allows to make each implementation into a single buffer
and to choose which one you'd like to use (or make ones which are tuned).

## List of base buffers (composite leafs)
  1. `InlineBuffer`: a buffer with an underlying fixed-size array. It cannot be
  resized. This makes it possible to use on the stack.
  2. `HeapBuffer`: a buffer that uses `std::ptr` to dynamically allocate and
  grow.
  3. `ZstBuffer`: a buffer for zero-sized types (ZST) only. It's a ZST itself.
  4. `AllocatorBuffer`: a buffer that uses an allocator to dynamically allocate
  and grow. It requires the `allocator` feature (enabled by default) since it
  uses the unstable allocator API.

## List of composite buffers
  1. `ZstoBuffer` (Zero-Sized Type Optimization): Optimization that uses
  `ZstBuffer` whenever T is a ZST, or its child otherwise.
  2. `SvoBuffer` (Small Vector Optimization): Have a small inline buffer but can
  grow into a bigger one (its child). This prevents allocations on small
  vectors.
  3. `ExponentialGrowthBuffer`: When trying to grow it will grow to the smallest
  power of 2 at least as big as the requested value. Useful to not allocate at
  every push.
  4. `AtLeastBuffer`: Specifies that when growing will at least grow to a set
  size.

There are also a few others that are utilities to make other buffers or for
testing.

## Collections
For now I've only implemented `Vector`. It's basicalle `Vec` but with a buffer.

## How it works
A `Buffer` implementation have four types of member functions:
  - Show the capacity (`capacity`)
  - Manage data (`read_value`, `write_value`, `manually_drop`)
  - Resizing (`try_grow`, `try_shrink`)
  - Utils which have default implementations. Allows override when knowledge
  allows optimizations (for example moving values when the data is contiguous)

This abstraction only assumes that the elements can be references by a `usize`
index, so the underlying mechanism could be a lot of things.

## Nightly
The code currently requires the nightly compiler because of
[`dropck_eyepatch`](https://github.com/rust-lang/rust/issues/34761)
for [Drop Check (Rustonomicon)](https://doc.rust-lang.org/nomicon/dropck.html)

There is an `allocator` feature to enable an allocator-based buffer. It also
requires nightly.

## Lack of code optimization
There are currently no optimizations of the code. This is because the effect of
the layout is too strong (I saw very big swings where should be none). It's
obvious that it needs to be optimized, probably by sprinkling `inline` and
`cold` everywhere, but I need to make sure it actually does what I think it does.

I plan to tackle this next, but it will require to make tools that randomize
the layout (probably based on [Stabilizer](https://github.com/ccurtsinger/stabilizer))

Test case showing how two staticlib crates can define a global_allocator each
and end up using the other's when everything is linked together, with neither
the rust compiler nor the linker raising an eyebrow.

Some details:
-------------

The foo crate is a staticlib that exports a single function, `foo`, that leaks
a `Box` allocated with its global_allocator. Its global_allocator wraps
`std::alloc::System` and prints "foo" to stdout on allocation.

The bar crate is exactly the same code, except it exports a single function `bar`,
and its global_allocator prints "bar".

The `foobar` executable is linked with libfoo and libbar in that order, and its
`main` function calls the exported function from each library.

The `barfoo` executable is linked with libbar and libfoo in that order, and its
`main` function calls the exported function from each library.

With the first executable, the global_allocator from libfoo is used for both
libraries, and with the second, the global_allocator from libbar is used.

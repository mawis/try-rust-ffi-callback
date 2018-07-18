Trial for passing Rust objects through C code
=============================================


This is some stripped down code I used while figuring out why some more complex
code I wrote did not compile. With this stripped down example I tested, that the
problem is not inside my FFI usage, and that this code does compile (and work).

There is no really useful code in it. I keep the repository as a reference to
myself on how to pass data through the FFI as the example in the rust book
expects to have the standard library available (using c_void, Option, and Box
from there).


Branches
--------

In order of creation:

*with_std* is the normal case of doing this when the standard library is
available.

*no_std* I removed the dependency to the standard library.

*no_std_cortex_m4* I checked that there is no difference, when I target a
Cortex M4 processor.

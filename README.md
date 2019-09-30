# Myopic
My optics library, Myopic, is a possibly bad implementation of the concept
of a functional lense. This is a beautiful concept, and if you
want an elegant implementation see [lens](http://hackage.haskell.org/package/lens)
from the Haskell ecosystem, and one of the many great introductions such
as [lens over tea](https://artyom.me/lens-over-tea-1).


Myopic is not elegant- it satisfies a need that I have for an implementation
of lense that acts on mutable data, which has no or little performance 
penalty, and which is simple enough to use in other libraries of mine.

# Concepts
This library contains three implementations of the concept of lenses. They
are all the simplest kind of lenses which can only get and set data
in structures (not the fancy Haskell ones which can do much more).


The lenses are implemented as struct containing either closures (lens), boxed
trait objects (lens\_box), or function pointers (lens\_fn).


Each lense implementation has a Lens type containing functions for getting
and setting data, and a ComposedLens type for nested compositions of lenses
and ComposedLens. This implementation comes from the [lenses](https://crates.io/crates/lenses)
library (credit where credit is due).


The reason there are three implementations was to benchmark them, as described
below. The takeaway is that the 'lens' module is at worst much faster, and
at best much much faster then the others.

# Benchmarking
The benchmarks can be run with cargo bench
```bash
cargo bench
```
They show that the 'lens' module is by far the fastest. It appears to be able 
to inline such that I have to use black\_box in criterion just to get any
benchmarking data out of them.


The other implementations seem to incur some indirection penalty which doesn't get
inlined (I assume). This appears to get worst the more they are composed, while
a level of composition in 'lens' does not incur a noticable penalty.

# The Name
The joke in the name is that this is both my own optics library, and it is
likely a bad optics library. Myopic means having bad vision, and it sounds
like "my optics". Hopefully this is a little amusing.


# Comparison
There are a number of Rust lense libraries, some of which are quite 
extensive, and some of which have vastly different designs.


 * [lenses](https://crates.io/crates/lenses) is very simple and is the library
   I started to use, which lead me to eventually create Myopic. This library
	 uses a Lens trait with one implementation for the Lens type, and one
	 for a type which contains composed lenses. This is the design I ended up
	 using in Myopic based on what I found in lenses.
	 I haven't benchmarked this, but lenses may be just as fast as
	 Myopic. I had trouble getting the specific use case covered where I want
	 to be able to modify data that may not have an address, and wanted
	 to mutate existing data. Perhaps lenses can do this, but I don't know how.
 * [photonix](https://crates.io/crates/photonix) is pretty interesting- its
   a completely different implementation using only traits, and not requiring
	 keeping closures around in memory. It seems to allow reaching into nested
	 structures and building up access into those structures (with macros as
	 far as I can tell).
	 Cool.
 * [refraction](https://crates.io/crates/refraction) is pretty extensive,
   breaking the concepts involved into many small pieces and buliding upwards.
	 It contains additional concepts like Prisms and is overall much better engineered
	 then Myopic.
	 It does seem confusing to me, given the number of traits involved, although I
	 think any lens implementation in Rust will reuqire this.
 * [shoggoth](https://crates.io/crates/shoggoth) is also cool, and seems
   extensive. At its own admission (given the name) it is quite complex. I wanted
	 something simple that I understood, so I didn't go down this route to madness.
 * [fp-core](https://crates.io/crates/fp-core) has only a lens trait and I don't believe
   it supports my use-case of mutating data in-place.
 * [rustz](https://crates.io/crates/rustz) has a better implementation then Myopic, which
   takes a slightly different approach to setting. I believe it precludes me from updating
	 data in-place, but if that doesn't turn out to matter then rustz may be the best option
	 in the sense that it is similar and the implementation is not as opaque as some.

# License
Myopic is licensed under either MIT or APACHE2, whichever you prefer.

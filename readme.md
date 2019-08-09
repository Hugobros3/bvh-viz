# bvh-viz

## A BVH Visualisation tool in Rust

Lately I wanted to try out rust and needed a small project to get myself familiar with it. I decided on a BHV complexity 
visualisation, since I needed one for another project anyway. I hit a few roadblocks and ended up doing a Kotlin version
to meet the deadline for the other project, but this is a good thing: I can benchmark the two against each other !

This visualisation currently only supports the BVH format used in [Rodent](https://github.com/anydsl/rodent), but I
 somewhat plan on adding support for more formats, and internally I don't have hard assumptions about anything.
 
## Feelings on Rust

### The good

 * Cargo and the crates ecosystem is something every single language should have in 2019. CMake is a pathetic excuse in comparison.
 * Sensible and mostly complete stdlib
 * Borrow checking makes sense most of the time and is certainly an idea worth trying out
 * The modules system is nice
 * No C++ feature-bloat insanity, the language is mostly neat and tidy without a billion ways to do something
 * [derive(Debug/Display)]
 
### The meh

 * The language syntax is mostly pleasant, but every now and then there's a hard edge: I don't like lifetime syntax (or lifetimes the feature really), and requiring braces for if/else is ugly imo.
 * CLion trips over itself with code suggestions more often than i'd like
 * Compile times
 * Compile error messages
 * Most cool features are `unstable` or at RFC stage

### The bad

 * Self-refential structs are incredibly hard and require `unsafe` at some point or another
 * IDE support in general is really nothing like what it is with something like Kotlin or Java
 * ... While the borrow checker can do work for you it's also easy to waste time by fighting it and lifetime-related error messages aren't that helpful and often confusing.
 * Bounds-checking everywhere by default somewhat clashes with the notion of high-performance, especially in tight loops
 * Why aren't enums variants types themselves ? Wtf ?
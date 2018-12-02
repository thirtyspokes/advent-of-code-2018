Solving Day Two
===============
Day two is a classic advent of code problem.  Part one is pretty straightforward, but part two adds an additional requirement that greatly increases the overall complexity.

Tests!
======
This was the first time I included tests in my solution, because I was pretty sure I would be doing a lot of refactoring after the fact.  The cargo tool includes a test runner and the assertions you would probably expect are all there.  I also find some tests helpful for the advent of code problems that involve some kind of rules about a string or collection just so I don't have to keep them all in my head as I evolve a solution.

The Rust Book has a few examples of writing tests, and it seems that in Rust you can put your tests in the same file as the code under test.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ... some tests
    #[test]
    fn a_test() {
        // test things
    }
}
```

The `#[cfg(test)]` directive tells Rust that the code that follows is a bunch of tests to be run by the test runner.  Each individual test gets the `#[test]` directive.  Additionally, it seems that you group all your tests in a sub-module inside of the main code - you can actually call the module whatever you want so long as the `cfg` directive is there, but convention seems to be to call it `tests`.

The `use super::*;` line is a neat one - our tests live inside of the greater module (main) that all of our other code is in as a sub-module.  Intuitively, it seems like it might make sense that sub-modules might have access to all of the code in their parent module, but they do not!  None of the functions in my solution are public (you make a function public by just putting `pub` in front of the `fn`) and so we need that `use super::*;` directive to say "give me access to everything in the parent module.

Having your tests live in the same file as the code under test is a new one for me - most languages I've used before have separate `src` and `tests` directories, like Clojure.  The closest I can think of is Go, where the test file lives in the same package directory as the code under test.  I'm not entirely sold on having it all be in the same file but you probably don't have to do it that way.

Also, VSCode with the rust extensions seems to understand Rust well enough to give me a `run test` link in the editor for each test that I write.  Nice!

Structs
=======
This is the first time I've used a struct in a solution, which are basically the same lightweight abstractions they are in Go.  I probably could have used just a tuple here, but I think it's useful to be able to distinguish one item from another via the type system, versus having to remember that the first int in the tuple is one thing and the second int is a different thing.

There's another reason to use a struct instead of a tuple here - one of the neat features of Rust that you'll hear about over and over again is "zero-cost abstractions".  In practice, this means that it doesn't "cost" anything in terms of performance or memory usage to use a higher-level abstraction in place of a low-level solution.  

If we compare this to Java or Go, using a struct here (or a class) instead of primitives involves more allocations and takes up more resources.  Since Rust has zero-cost abstractions, using the tuple should be equally as performant as using the struct so the benefit we get of having named fields in a struct is totally free!

I have to admit that the "cost" of an abstraction is very rarely something I think about, probably because I don't write low-level or performance-sensitive code all that often.  However, this is probably one of the reasons people love Rust so much - since the abstractions cost nothing, you can just write the code that's easier to maintain or read and not worry about how much slower the code will be.

Defining a struct looks basically the same as it does in Go:

```rust
#[derive(Debug,PartialEq)]
struct Score {
    has_two: isize,
    has_three: isize,
}
```

The exception is that `derive` directive up above it - what gives? Rust has Traits, which are kind of like an interface that your struct can implement.  For my `Score` struct, I need to implement `Debug` (to allow it to be printed in a string) and `PartialEq` (to allow it to be compared to other instances of `Score`).

In many cases, you would implement the trait by providing an implementation "by hand".  So if we want to implement `Debug` for `Score`, we have to provide code that tells Rust how to write an instance of `Score` as a string:

```rust
impl fmt::Debug for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score {{ has_two: {}, has_three: {} }}", self.has_two, self.has_three)
    }
}
```

However, many frequently-used traits can be implemented automatically without having to write code - that's what the `Derive` directive is saying.  `Derive` lets you provide a list of traits for your struct and will implement them automatically if possible.  Pretty cool!

Interestingly, the only reason I even need those two traits on `Score` is because of the tests - if I didn't have any tests in the file I could remove that `Derive` because there's no code that tries to compare one `Score` to another or print one to a string.

Zip
===
`zip` is a cool sequence function that Haskell and Clojure both have that will let you iterate pairwise over two sequences at once.  My algorithm for finding which strings were "nearly identical" was "iterate over each character in each string as pairs, and allow exactly one difference between the pairs."  My original solution looked like this:

```rust
for (i, a_char) in a.chars().enumerate() {
    // go over each character in string a,
    // and compare it to the character at the
    // same index in b
    if a_char != b_vec[i] {
        if found_difference {
            return false;
        } else {
            found_difference = true
        }
    }
}
```

With `zip`, that can be simplified and doesn't have to be index-aware:

```rust
for (a_char, b_char) in a.chars().zip(b.chars()) {
    if a_char != b_char {
        if found_difference {
            return false;
        } else {
            found_difference = true
        }
    }
}
```

Strings versus &strs
====================
The idea of a "string" is something you have to think about a lot more in Rust than in any other language I've used.  There's two many ways of representing that - `std::string::String` and `&str`.

`std::string::String` is actually a struct, not a primitive, and it owns the actual contents of the string itself.  It can be mutable, so the size of the string isn't necessarily known at compile-time.  `String`s are allocated on the heap.  Many folks equate a `String` with something like a `StringBuffer` in Java - it has a length and a capacity which can both change.

`&str` is a borrowed string primitive. A `&str` isn't guaranteed to be allocated on the heap. It's best thought of as an immutable view into a particular string (a string slice).  Code can't take ownership of a '&str', so you of course can't mutate them.

Knowing when to use which is kind of confusing at first, but based on this info and a nice summary of the rules from Christian again, it basically seems to boil down to this:

- If you're returning it from a function, it needs to be a `String` so the calling code can own it.
- If you're using it as a function parameter and you don't need to mutate it, then you don't need to own it, so make it a `&str`.
- If you're using it as a function parameter and you do need to mutate it, it has to be a `String`.

In general, preferring `&str` as function parameters will usually work because a borrowed `String` (i.e., `&String`) can be used as a `&str`.

As a final note, why the hell does it matter where in memory the string might be allocated?  Stack memory is like the data structure of the same name - it's last-in, first out (LIFO).  Storing stuff on the stack is therefore super-fast because you don't have to look for a value - it's right there on top of the stack.  However, to put something on the stack, you also have to know in advance what the size of the data is.  Heap memory, by contrast, involves holding on to an address for your value and following that pointer whenever you need the data, which is slower.  But heap memory can be dynamic in size - you don't necessarily have to know in advance how big the data will be.
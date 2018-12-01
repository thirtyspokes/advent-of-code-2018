Solving Day 1
=============
The first part of day one is a pretty simple problem, so it was a great first crack at using Rust to do things other than Hello World.  Like previous years we need to read some stuff out of a file and then run some calculations on it.  The rust book has some usage of basic filesystem I/O, so I knew that most of what I wanted would be in `std::io` and `std::fs`.  

I think the most important thing I learned from Rust is that the compiler is super helpful, but almost TOO helpful.  Many times, I was stumped over why it hated some code I'd written but it would tell me exactly how to fix it.  This would get the code working without me understanding why it didn't work to begin with!

I think with experience this will make life easier on a day-to-day basis, but for now, I need to be sure to understand what the compiler is complaining about before making the changes it suggests.

Options and Results
===================

Most of these operations, since they have to talk to the scary outside world, can fail and will therefore return `Result` or `Option` types - googling around how to use this stuff quickly led me to learning the `?` operator.  This is a nice little shortcut if you're doing stuff that can result in a `Result` or `Option` inside of a function that's defined as returning the same.

```rust
// This code:
match some_fn_returning_option(reader) {
    Ok(result) => return result,
    Err(e) => , return Err(e.into())
}

// Can just be shortened to:
return some_fn_returning_option(reader)?
```

Since this is basically just toy code and I don't yet know enough to care about handling errors the smart way, I ended up not using a lot of this pattern and just calling `unwrap()` on anything that was returning a `Result` back - `unwrap()` will basically just extract the value out of the option for you, but panic if it's `None` which makes it unsuitable for the Real World.

Parsing Strings into Numbers
============================
Rust can handily turn a string representing a number into an actual number with `parse()` - you tell it what you want the string to be turned into and it'll give you a `Result` that has either the converted value or an error if it wasn't possible.  My original code for handling this line parsing was this:

```rust
fn parse_line(line: String) -> (String, i64) {
    let sign = String::from(&line[0..1]);
    let amount = &line[1..].parse::<i64>().unwrap();
    (sign, *amount)
}
```

This demonstrates a few things:
- You can index on Strings with range indexes (but not just numbers - line[0..1] is ok, but line[0] isn't).
- Once you parse things out you again would normally handle any potential errors, but I again went the lazy route.
- Since Rust has expressions, to return a tuple of the results from parsing I just need to put it at the end of the function.

This worked just fine, but I later learned that `parse` is actually intelligent enough to know how to parse a string integer that's signed - so `"+1"` and `"-23"` will be correctly parsed into a positive or negative number.  With that in mind I ultimately dropped this function completely and just used parse in-place.

Lifetimes
=========
When I wrote `parse_line` above for the first time, it looked like this:

```rust
fn parse_line(line: String) -> (String, &i64) {
    let sign = String::from(&line[0..1]);
    let amount = &line[1..].parse::<i64>().unwrap();
    (sign, amount)
}
```

I started out by returning a String and then a pointer to the int amount result, but the Rust compiler actually hated this: "this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments".  What the heck does that mean?

I remember the very lightweight introduction to lifetimes at the start of the Rust book tells us that Rust doesn't have a garbage collector.  It will remove references as they go out of scope and are unused, but in this case, it seems that the compiler can't be sure when the pointer to the returned integer will actually become unused.  This means that it wouldn't ever really be safe to free that memory!

This seemed like I was doing something dumb - it's just a primitive int, right?  So I updated the signature to be `i64` instead of `&i64` and got a different error message about a type mismatch that suggested simply dereferencing the the int.  This was in the end what I wanted - just to return the integer value, not a pointer to anything.  This probably shouldn't have tripped me up so much, but even in Go the pointer semantics are usually ignorable - in Rust, I definitely need to be conscious of whether I care about the value of a thing or the actual thing itself.

Iterators
=========
I pretty quickly found `std::io::BufReader::lines()` for reading through a file line by line.  This returns each line of the file as a `Result`, and I knew that I had a couple of transformations I wanted to compose onto that iterator to deal with these `Results` and parsing the strings.

My first attempt looked like this:

```rust
for line in reader.lines() {
    let (sign, amount) = parse_line(line?);
    if sign == "-" {
        frequency -= amount
    } else {
        frequency += amount
    }
}
```

This uses the iterator as basically a for loop a la Python or PHP.  As I read through the docs more, I found that Rust supports most of the same sequence functions I'm used to from Clojure!  Finding `fold`, I was able to condense all of that down into a one-liner:

```rust
reader.lines().fold(0, |sum, line| sum + line.unwrap().parse::<isize>().unwrap())
```

Now, this may be shorter, but it's not as readable.  I saw that there was a `sum` iterator function that would give me the final result without having to use `fold`, but I thought this might result in multiple iterations over the whole sequence to build the result.  

In Clojure, you can usually make a tranducer out of a series of sequence operations such that they'll be executed in a single pass:

```clojure
;; Applying the function x over the results 
;; of mapping function y over the results of
;; mapping the function z over coll...
(map x (map y (map z coll)))

;; Will be transformed into this, which avoids multiple
;; traversals of coll:
(map (comp x y z) coll)
```

After talking to my buddy Christian who's a rust expert, it turns out that Rust's iterators will do the same thing since they're lazily computed!  Thus, my final effort was equally as performant and a lot more readable to my newbie eyes:

```rust
reader.lines()
    .map(|line| line.unwrap())
    .map(|change| change.parse::<isize>().unwrap())
    .sum()
```

Solving Part Two
================
Part two was an interesting twist - we needed to basically apply the changes forever until we found a frequency that appeared twice.  Rust does have a `cycle` function that would have let me iterate over the changes forever until I found the result, just like Python and Clojure, but I instead opted to achieve this by seeking to the beginning of the file and read through it over and over.  I don't know what's actually faster, but in theory this prevents me from ever loading the entire file into memory, which is cool.

This was also my first time using a `HashMap`, but they basically work exactly like I expect them to.
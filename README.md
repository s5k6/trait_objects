
Hello,

I have some question about trait objects of combined traits.

They need some build up, you'll find them at the very end.  A
[complete minimal example is on GitHub][1], runs with `cargo run`.

I've been playing with trait objects, which seem to solve a similar
problem as Haskell's existential types: They allow you to put things
of *different type* (`A` and `B`) into a generic datastructure like
`Vec<T>`, which would normally force you to choose `T`=`A`, or
`T`=`B`, exclusively, or use a sum type like `Either`.

However, if `A` and `B` both implement a common trait, say:
`Printable`, then you may create a `Vec<&dyn Printable>` which can
hold items of both types.

    let a = A{};
    let b = B{};
    let items: Vec<&dyn Printable> = vec!(&a, &b);

This is type safe, since the compiler only allows you to use those
properties of the elements of the vector that are provided by the
common trait.

In other words: When taking an item from the vector, all you know for
sure is it's printable.  That's what the type says, and that's what
the compiler allows you to do.  You may print them.

So far, so good.

I have then tried (obviously) to reconstruct the original type of the
vector's members, and this is also possible: Define an enum for every
possible case, here this would be

    enum Discovery<'a> { IsA(&'a A), IsB(&'a B) }

extend `A` and `B` with a function to report their case

    trait Discoverable { fn discover(&self) -> Discovery; }

    impl Discoverable for A { fn discover(&self) -> Discovery { IsA(&self) } }
    impl Discoverable for B { fn discover(&self) -> Discovery { IsB(&self) } }

and almost!  Next I wanted to create a vector of type

    Vec<&dyn Printable + Discoverable>

which failed with “only auto traits can be used as additional traits
in a trait object”, but included a helpful suggestion (Thanks!)
making me try

    trait CombinedTrait: Printable + Discoverable {}
    impl<T: Printable + Discoverable> CombinedTrait for T {}

    let items: Vec<&dyn CombinedTrait> = vec!(&a, &b);

Now this works partially.  The function

    fn print_and_discover<T: CombinedTrait>(x: &T) {
        x.print();
        match x.discover() {
            IsA(a) => a.only_a(),
            IsB(b) => b.only_b(),
        }
    }

happily prints the items using their common “printability”, then
discovers their type and dispatches accordingly, calling functions
`only_` that are ony implemented for the individual types.

But this seems to go only so far.  While I can create my vector

    let items: Vec<&dyn CombinedTrait> = vec!(&a, &b);

print and rediscover the items

    items[i].print();
    match items[i].discover() { … }

I fail to pass the vector to a function:

    fn print_list(items: &Vec<&dyn Printable>) { … }
    print_list(&items);

gives “expected trait `Printable`, found trait `CombinedTrait`”.

**Question 1:** Why does a *found* `CombinedTrait` not satisfy an
*expected* `Printable`?

Also, I fail to create and use an iterator (other than the counting
loop above):

    for i in items.iter() { print_and_discover(i); }

gives “the trait bound `&dyn CombinedTrait: CombinedTrait` is not
satisfied”

**Question 2:** is it not?  should it not be?

Cheers!


[1]: https://github.com/s5k6/trait_objects

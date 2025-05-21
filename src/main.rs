
struct A {}

impl A {
    fn only_a(&self) {
        println!("only A");
    }
}


struct B {}

impl B {
    fn only_b(&self) {
        println!("only B");
    }
}



trait Printable {
    fn print(&self);
}

impl Printable for A {
    fn print(&self) {
        println!("print A");
    }
}

impl Printable for B {
    fn print(&self) {
        println!("print B");
    }
}

fn print_list(items: &Vec<&dyn Printable>) {
    for i in items {
        i.print();
    }
}



enum Discovery<'a> {
    IsA(&'a A),
    IsB(&'a B)
}
use Discovery::*;

trait Discoverable {
    fn discover(&self) -> Discovery;
}

impl Discoverable for A {
    fn discover(&self) -> Discovery { IsA(&self) }
}

impl Discoverable for B {
    fn discover(&self) -> Discovery { IsB(&self) }
}

trait CombinedTrait: Printable + Discoverable {}

impl<T: Printable + Discoverable> CombinedTrait for T {}

fn print_and_discover<T: CombinedTrait>(x: &T) {
    x.print();
    match x.discover() {
        IsA(a) => a.only_a(),
        IsB(b) => b.only_b(),
    }
}



fn main() {

    let a = A{};
    let b = B{};


    // Nice!

    println!("-- different types in one list");
    let items: Vec<&dyn Printable> = vec!(&a, &b);
    print_list(&items);


    // Quality!!!

    println!("-- reconstruction of original type");
    print_and_discover(&a);
    print_and_discover(&b);


    // Looking good...

    println!("-- combined trait in vector (commented out)");

    #[allow(unused_variables)]
    let items: Vec<&dyn CombinedTrait> = vec!(&a, &b);

    for i in 0..items.len() {
        items[i].print();
        match items[i].discover() {
            IsA(a) => a.only_a(),
            IsB(b) => b.only_b(),
        }
    }


    // These two don't compile, and I don't understand why.

    /*
    print_list(&items);
    // */

    /*
    for i in items.iter() {
        print_and_discover(i);
    }
    // */
}

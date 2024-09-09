// allign in file functions
// 1. trait definitions
// 2. data structures and trait implementations
// 3. functions
// 4. main

fn pure_function1(x: u32) -> u32 {
    x * x
}

fn impure_function(x: u32) -> u32 {
    println!("x = {}", x);
    x * x
}

// monads
trait Monad<A> {
    fn return_(t: A) -> Self;
    // :: A -> Monad<A>

    fn bind<MB, B>(m: Self, f: dyn Fn(A) -> MB) -> MB
    where
        MB: Monad<B>;
    // :: Monad<A> -> (A -> Monad<B>) -> Monad<B>
}

// Function currying
fn not_curried(p1: u32, p2: u32) -> u32 {
    p1 + p2
}

fn curried(p1: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |p2: u32| p1 + p2)
}

// Memoization
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

cached! {
    FIB;
    fn fib(n: u64) -> u64 = {
        if n == 0 || n == 1 { return n; }
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    // functional composition
    let fsin = |x: f64| x.sin();
    let fabs = |x: f64| x.abs();
    // feed output of one into the other
    let transform = |x: f64| fabs(fsin(x));

    // functors (take data, mutate it and return mutated data(nor changing original data, as I understand)
    let mut c = 0;
    for _ in vec!['a', 'b', 'c'].into_iter().map(|letter| {
        c += 1;
        (letter, c)
    }) {}

    // Function currying
    not_curried(1, 2);
    curried(1)(2);

    // lazy evaluation
    let x = {
        println!("side effect");
        1 + 2
    };
    let y = || {
        println!("side effect");
        1 + 2
    }; // <- lazy

    // memoization
    fib(30);
}

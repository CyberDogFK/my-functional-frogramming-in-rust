#[macro_use]
extern crate metaderive;

pub trait TypeName {
    fn typename() -> String;
    
    fn attributes(&self) -> String;
}

#[derive(TypeName)]
struct MyStructA {
    a: u32,
    b: f32
}

// so, if we want to add few more derives, we just can 'write' one more impl block
impl MyStructA {
    fn additional_impl() -> String {
        "impl".to_string()
    }
}

impl MyStructA {
    fn some_more_impl(&self) -> String {
        "some_more-impl".to_string()
    }
}

macro_rules! my_vec_macro {
    ( $( $x:expr ),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    }
}

macro_rules! my_macro_branch {
    (1 $e:expr) => (println!("mode 1: {}", $e));
    (2 $e:expr) => (println!("mode 2: {}", $e));
}

enum DSLTerm {
    TVar { symbol: String },
    TAbs { param: String, body: Box<DSLTerm> },
    TApp { f: Box<DSLTerm>, x: Box<DSLTerm> }
}

macro_rules! dsl {
    ( ( $( $e:tt ) * ) ) => (dsl!( $( $e )* ));
    ( $e:ident ) => (DSLTerm::TVar {
        symbol: stringify!($e).to_string()
    });
    ( fn $p: ident . $b:tt ) => (DSLTerm::TAbs {
        param: stringify!($p).to_string(),
        body: Box::new(dsl!($b))
    });
    ( $f:tt $x:tt ) => (DSLTerm::TApp {
        f: Box::new(dsl!($f)),
        x: Box::new(dsl!($x))
    });
}


fn main() {
    my_vec_macro!(1, 2, 3);
    my_macro_branch!(1 "abc");
    my_macro_branch!(2 "def");

    dsl!( a );
    dsl!( fn a . a );
    dsl!( f a );
    dsl!( (f a) );

    let tn = MyStructA { a: 1, b: 2.0 };
    println!("{}", MyStructA::typename());
    println!("{}", MyStructA::additional_impl());
    println!("{}", tn.attributes());
    println!("{}", tn.some_more_impl());
}
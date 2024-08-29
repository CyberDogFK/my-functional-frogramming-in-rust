enum Term {
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> }
}

trait Data1Trait {
    // constructors
    fn new(a: i32, b: f64, c: String) -> Self;
    
    // methods
    fn get_a(&self) -> i32;
    fn get_b(&self) -> f64;
    fn get_c(&self) -> String;
}

fn main() {
}

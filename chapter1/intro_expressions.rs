struct MyStruct {
    a: u32,
    b: f32,
    c: String
}

enum Term {
    TermVal { value: String},
    TermVar { sumbol: String },
    TermApp { f: Box<Term>, x: Box<Term> },
    TermAbs { arg: String, body: Box<Term> }
}

fn main() {
    let m1 = MyStruct {
        a: 1,
        b: 1.0,
        c: "".to_string()
    };

    let m2 = (1, 1.0, "".to_string());

    let mut t = Term::TermVar {
        sumbol: "v1".to_string()
    };
    let s = match t {
        Term::TermVal { value: v1} => v1,
        Term::TermVar { sumbol: v1} => v1,
        Term::TermApp { f: ref v1, x: ref v2} => "TermApp(?,?)".to_string(),
        Term::TermAbs { arg: ref mut v1, body: ref mut v2 } => "TermAbs(?,?)".to_string(),
    };
    println!("{}", s)
}

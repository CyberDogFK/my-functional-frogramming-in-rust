mod intro_expressions;

struct PointU32 {
    x: u32,
    y: u32,
}

struct PointF32 {
    x: f32,
    y: f32,
}

struct PointI32 {
    x: i32,
    y: i32,
}

struct Point<T> {
    x: T,
    y: T
}

fn foo_u32(x: u32) -> u32 {
    x * x
}

fn foo_f32(x: f32) -> f32 {
    x * x
}

fn foo_i32(x: i32) -> i32 {
    x * x
}

fn foo<T>(x: T) -> T
where T: std::ops::Mul<Output=T> + Copy
{
    x * x
}

fn bar<F, T>(f: F, x: T) -> T
where F: Fn(T) -> T
{
    f(x)
}

fn main() {
    let s = bar(|x| x * x, 5);
    println!("{}", s);

    let res = (0..10).map(|x| x * x)
        .inspect(|x| println!("value {}", *x))
        .filter(|x| *x<3)
        .filter_map(|x| Some(x)) // filter and maps with a single function
        .fold(0, |x, y| x + y); // fold reduces all results to a single value, starting from an initial value, working left to right.
}

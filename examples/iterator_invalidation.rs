#[cfg(feature = "broken")]
fn iterator_invalidation() {
    let mut v = vec![];
    v.push("Hello");
    let x = &v[0];
    v.push("world");
    println!("{}", x);
}

fn main() {
#[cfg(feature = "broken")]
iterator_invalidation();
}

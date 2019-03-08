// run-pass
// Test that overloaded calls work with zero arity closures

// pretty-expanded FIXME #23616

fn main() {
    let functions: [Box<Fn() -> Option<()>>; 1] = [Box::new(|| None)];

    let _: Option<Vec<()>> = functions.iter().map(|f| (*f)()).collect();
}

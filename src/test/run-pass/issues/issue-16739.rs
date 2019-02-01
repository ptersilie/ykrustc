// run-pass
#![feature(box_syntax)]
#![feature(unboxed_closures, fn_traits)]

// Test that unboxing shim for calling rust-call ABI methods through a
// trait box works and does not cause an ICE.

struct Foo { foo: u32 }

impl FnMut<()> for Foo {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> u32 { self.foo }
}

impl FnOnce<()> for Foo {
    type Output = u32;
    extern "rust-call" fn call_once(mut self, _: ()) -> u32 { self.call_mut(()) }
}

/////////////////////////////////////////////////////////////////////////

impl FnMut<(u32,)> for Foo {
    extern "rust-call" fn call_mut(&mut self, (x,): (u32,)) -> u32 { self.foo + x }
}

impl FnOnce<(u32,)> for Foo {
    type Output = u32;
    extern "rust-call" fn call_once(mut self, args: (u32,)) -> u32 { self.call_mut(args) }
}

/////////////////////////////////////////////////////////////////////////

impl FnMut<(u32,u32)> for Foo {
    extern "rust-call" fn call_mut(&mut self, (x, y): (u32, u32)) -> u32 { self.foo + x + y }
}

impl FnOnce<(u32,u32)> for Foo {
    type Output = u32;
    extern "rust-call" fn call_once(mut self, args: (u32,u32)) -> u32 { self.call_mut(args) }
}

fn main() {
    let mut f = box Foo { foo: 42 } as Box<FnMut() -> u32>;
    assert_eq!(f.call_mut(()), 42);

    let mut f = box Foo { foo: 40 } as Box<FnMut(u32) -> u32>;
    assert_eq!(f.call_mut((2,)), 42);

    let mut f = box Foo { foo: 40 } as Box<FnMut(u32, u32) -> u32>;
    assert_eq!(f.call_mut((1, 1)), 42);
}

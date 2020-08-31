struct Foo {
    a: &'static mut [u32],
}

impl Foo {
    pub fn ez2(&mut self, input: u32) -> &mut [u32] {
        if input < 5 {
            self.a
        } else {
            &mut []
        }
    }

    pub fn ez3(&mut self, input: u32) -> usize {
        if self.ez2(input).len() > 0 {
            1
        } else {
            panic!("abort");
        }
    }
}

static mut BUF: [u32; 2] = [1, 2];

pub fn main() {
    let mut foo = unsafe { Foo { a: &mut BUF } };
    let out = foo.ez3(2);
    println!("out: {}", out);
}

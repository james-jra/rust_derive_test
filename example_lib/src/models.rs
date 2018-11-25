use contents_len::ContentsLen;

pub struct Bar(u32);
impl ContentsLen for Bar {
    fn contents_len(&self) -> usize {
        4
    }
}

pub struct Baz(u8);
impl ContentsLen for Baz {
    fn contents_len(&self) -> usize {
        1
    }
}

#[derive(ContentsLen)]
pub struct Foo {
    bar: Bar,
    baz: Baz,
}
impl Foo {
    pub fn new(bar: Bar, baz: Baz) -> Foo {
        Foo { bar, baz }
    }
}

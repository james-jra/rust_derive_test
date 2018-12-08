use crate::contents_len::ContentsLen;

#[derive(Debug, PartialEq)]
pub enum Typ {
    Foo,
    Bar,
    Baz,
}

pub struct Bar(u32);
impl ContentsLen for Bar {
    fn typ(&self) -> Typ {
        Typ::Bar
    }
    fn contents_len(&self) -> usize {
        4
    }
}

pub struct Baz(u8);
impl ContentsLen for Baz {
    fn typ(&self) -> Typ {
        Typ::Baz
    }
    fn contents_len(&self) -> usize {
        1
    }
}

#[derive(ContentsLen)]
#[type_variant(Typ::Foo)]
pub struct Foo {
    bar: Bar,
    baz: Baz,
}
impl Foo {
    pub fn new(bar: Bar, baz: Baz) -> Foo {
        Foo { bar, baz }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo_typ() {
        let foo = Foo {
            bar: Bar(100),
            baz: Baz(20),
        };
        assert_eq!(foo.typ(), Typ::Foo);
    }

    #[test]
    fn test_foo_contents_len() {
        let foo = Foo {
            bar: Bar(100),
            baz: Baz(20),
        };
        assert_eq!(foo.contents_len(), 5);
    }
}

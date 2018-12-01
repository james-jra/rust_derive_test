use models::Typ;

pub trait ContentsLen {
    fn typ(&self) -> Typ;
    fn contents_len(&self) -> usize;
}

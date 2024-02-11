#[derive(Debug, Copy, Clone)]
pub enum Action {
    Raise(usize),
    Call,
    Fold,
}

pub mod states;

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum Event {
    Exit,
    Arm,
}

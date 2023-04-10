

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum AudioEvent {
    #[default]
    Menu,
    Ambient,
    Good,
    Bad,
}
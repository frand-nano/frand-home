pub trait StateMessage {
    fn error(err: String) -> Self;
}
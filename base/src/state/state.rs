pub trait State {
    type Property;
    type Message;
    fn apply(&mut self, message: Self::Message);
    fn export_to(&self, message: &mut Self::Message);
}
pub trait State: Send {
    type Property;
    type Message: Send;
}
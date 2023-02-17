pub trait GLObject {
    fn bind(&self);
    fn unbind(&self);
}
// might delete
pub trait GLObject {
    fn bind(&self);
    fn unbind(&self);
    fn handle(&self) -> u32;
}

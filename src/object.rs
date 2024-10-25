use std::fmt::Display;

#[derive(Debug)]
pub struct ObjRef<T: Display> {
    pub value: *mut T,
}

impl<T: Display> ObjRef<T> {
    pub fn new(value: *mut T) -> ObjRef<T> {
        Self { value }
    }
}

impl<T: Display> Copy for ObjRef<T> {}

impl<T: Display> Clone for ObjRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Display> Display for ObjRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = unsafe { &*self.value };
        write!(f, "{x}",)
    }
}

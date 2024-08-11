pub struct StateWrapper<T: 'static + Send + Sync> {
    inner: T,
}

impl<T: 'static + Send + Sync> StateWrapper<T> {
    pub fn new(value: T) -> Self {
        StateWrapper { inner: value }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
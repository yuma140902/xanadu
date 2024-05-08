pub trait Component: bytemuck::Pod {}

impl<T> Component for T where T: bytemuck::Pod {}

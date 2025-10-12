pub trait Pixel: Copy + Clone + Default {
    type Channel: Copy + Clone + Default;

    fn channel_count() -> usize;

    fn get_channel(&self, index: usize) -> Self::Channel;
    fn set_channel(&mut self, index: usize, value: Self::Channel);

    fn blend(&self, background: &Self) -> Self;
}


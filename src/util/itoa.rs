use itoa::Buffer;
use twilight_model::{gateway::ShardId, id::Id};

pub trait Itoa {
    fn itoa(&self) -> String;
}

impl<T> Itoa for Id<T> {
    fn itoa(&self) -> String {
        let mut buffer = Buffer::new();

        let size = self.get().ilog10() + 1;

        let mut format = String::with_capacity(size as usize);
        format.push_str(buffer.format(self.get()));

        format
    }
}

impl Itoa for ShardId {
    fn itoa(&self) -> String {
        let mut buffer = Buffer::new();

        let size = self.number().checked_ilog10().map_or_else(|| 1, |n| n + 1)
            + self.total().checked_ilog10().map_or_else(|| 1, |n| n + 1)
            + 1;

        let mut format = String::with_capacity(size as usize);
        format.push_str(buffer.format(self.number()));
        format.push('/');
        format.push_str(buffer.format(self.total()));

        format
    }
}

use itoa::Buffer;
use twilight_model::gateway::ShardId;

pub trait Itoa {
    fn itoa(&self) -> String;
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

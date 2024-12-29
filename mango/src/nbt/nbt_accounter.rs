use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub struct NbtAccounter {
    quota: u64,
    usage: AtomicU64,
    max_depth: u32,
    depth: AtomicU32,
}
impl NbtAccounter {
    fn new(quota: u64, max_depth: u32) -> Self {
        Self {
            quota,
            usage: AtomicU64::new(0),
            max_depth,
            depth: AtomicU32::new(0),
        }
    }

    /// Allocate an NbtAccounter on the heap
    pub fn create(quota: u64) -> Box<Self> {
        Box::new(Self::new(quota, 512))
    }

    pub fn account_bytes(&self, usage: u64) {
        let prev = self.usage.fetch_add(usage, Ordering::Relaxed);
        if prev + usage > self.quota {
            panic!("Tried to read NBT tag that was too big; tried to allocate: {} + {} bytes where max allowed: {}", prev, usage, self.quota);
        }
    }

    pub fn push_depth(&self) {
        let prev = self.depth.fetch_add(1, Ordering::Relaxed);
        if prev >= self.max_depth {
            panic!(
                "Tried to read NBT tag with too high complexity, depth > {}",
                self.max_depth
            );
        }
    }

    pub fn pop_depth(&self) {
        let prev = self.depth.fetch_sub(1, Ordering::Relaxed);
        if prev <= 0 {
            panic!("NBT-Accounter tried to pop stack-depth at top-level");
        }
    }
}

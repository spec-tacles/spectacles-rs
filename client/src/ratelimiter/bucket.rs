use std::collections::vec_deque::VecDeque;

use futures::sync::oneshot::{self, Receiver, Sender};

// use parking_lot::Mutex;

/// A Ratelimiter bucket used for maintaining Discord ratelimits.
pub struct RatelimitBucket {
    /// The remaining time left for this request.
    pub remaining: i64,
    /// The request limit.
    pub limit: i64,
    /// Amount of milliseconds until reset.
    pub reset: i64,
    /// Where queued requests are held.
    pub queue: VecDeque<Sender<()>>,
    pub timeout: bool,
}

impl Default for RatelimitBucket {
    fn default() -> Self {
        Self {
            limit: 9223372036854775807,
            queue: VecDeque::new(),
            remaining: 9223372036854775807,
            reset: 9223372036854775807,
            timeout: false,
        }
    }
}

impl RatelimitBucket {
    fn take(&mut self) -> Option<Receiver<()>> {
        if self.reset == 0 {
            let (tx, rx) = oneshot::channel();

            self.queue.push_back(tx);

            Some(rx)
        } else {
            None
        }
    }
}

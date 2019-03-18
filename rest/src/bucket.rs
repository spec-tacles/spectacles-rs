use std::collections::vec_deque::VecDeque;

use parking_lot::Mutex;

/// A Ratelimiter bucket used for maintaining Discord ratelimits.
pub struct RatelimitBucket {
    /// The remaining time left for this request.
    pub remaining: i64,
    /// The request limit.
    pub limit: i64,
    /// Amount of milliseconds until reset.
    pub reset: i64,
}


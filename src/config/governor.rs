use actix_governor::{Governor, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;

pub fn rate_limiter() -> Governor<PeerIpKeyExtractor, NoOpMiddleware> {
    // configure rate limiting
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2) // Allow 2 requests per second
        .burst_size(3) // Allow bursts of up to 3 requests
        .finish()
        .unwrap(); // You might want to handle this error more gracefully

    Governor::new(&governor_config)
}

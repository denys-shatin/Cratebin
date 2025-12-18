use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Simple in-memory rate limiter
#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
    max_requests: usize,
    window: Duration,
}

struct RateLimiterState {
    requests: HashMap<IpAddr, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                requests: HashMap::new(),
            })),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self, ip: IpAddr) -> bool {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();

        // Get or create request history for this IP
        let requests = state.requests.entry(ip).or_insert_with(Vec::new);

        // Remove old requests outside the window
        requests.retain(|&time| now.duration_since(time) < self.window);

        // Check if limit exceeded
        if requests.len() >= self.max_requests {
            return false;
        }

        // Add current request
        requests.push(now);
        true
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Response {
    // For now, we'll skip rate limiting in middleware
    // In production, extract IP from request and check rate limiter
    // This is a placeholder that always allows requests
    next.run(req).await
}

/// CORS middleware configuration
pub fn cors_layer() -> tower_http::cors::CorsLayer {
    use tower_http::cors::{Any, CorsLayer};

    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new(5, 60);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // Should allow first 5 requests
        for _ in 0..5 {
            assert!(limiter.check(ip));
        }
    }

    #[test]
    fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new(3, 60);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // Allow first 3 requests
        for _ in 0..3 {
            assert!(limiter.check(ip));
        }

        // Block 4th request
        assert!(!limiter.check(ip));
    }

    #[test]
    fn test_rate_limiter_different_ips() {
        let limiter = RateLimiter::new(2, 60);
        let ip1 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ip2 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));

        // Each IP should have independent limits
        assert!(limiter.check(ip1));
        assert!(limiter.check(ip1));
        assert!(!limiter.check(ip1)); // ip1 blocked

        assert!(limiter.check(ip2)); // ip2 still allowed
        assert!(limiter.check(ip2));
        assert!(!limiter.check(ip2)); // ip2 blocked
    }
}

// **Feature: cratebin, Property 15: Rate limiting enforcement**
// For any client making requests, after exceeding the rate limit threshold, subsequent requests should be rejected with a rate limit error.

#[cfg(test)]
mod property_tests {
    use crate::middleware::RateLimiter;
    use quickcheck::QuickCheck;
    use std::net::{IpAddr, Ipv4Addr};

    fn prop_rate_limit_enforcement(limit: u8, extra_requests: u8) -> bool {
        let limit = limit.max(1) as usize; // At least 1
        let extra = extra_requests.max(1) as usize; // At least 1 extra

        let limiter = RateLimiter::new(limit, 60);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // Make requests up to limit - should all succeed
        for _ in 0..limit {
            if !limiter.check(ip) {
                return false; // Should not be blocked yet
            }
        }

        // Make extra requests - should all be blocked
        for _ in 0..extra {
            if limiter.check(ip) {
                return false; // Should be blocked
            }
        }

        true
    }

    fn prop_different_ips_independent(limit: u8) -> bool {
        let limit = limit.max(1).min(10) as usize; // Between 1 and 10

        let limiter = RateLimiter::new(limit, 60);
        let ip1 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ip2 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));

        // Exhaust limit for ip1
        for _ in 0..limit {
            limiter.check(ip1);
        }

        // ip1 should be blocked
        if limiter.check(ip1) {
            return false;
        }

        // ip2 should still be allowed
        limiter.check(ip2)
    }

    #[test]
    fn run_rate_limit_enforcement_tests() {
        QuickCheck::new()
            .tests(50) // Fewer tests as this involves timing
            .quickcheck(prop_rate_limit_enforcement as fn(u8, u8) -> bool);
    }

    #[test]
    fn run_independent_ip_tests() {
        QuickCheck::new()
            .tests(50)
            .quickcheck(prop_different_ips_independent as fn(u8) -> bool);
    }
}

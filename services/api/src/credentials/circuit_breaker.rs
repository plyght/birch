use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
struct CircuitBreakerState {
    state: CircuitState,
    failure_count: u32,
    last_failure_time: Option<DateTime<Utc>>,
    last_success_time: Option<DateTime<Utc>>,
    half_open_request_count: u32,
}

pub struct CircuitBreaker {
    states: Arc<Mutex<HashMap<String, CircuitBreakerState>>>,
    failure_threshold: u32,
    timeout_seconds: i64,
    half_open_max_requests: u32,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout_seconds: i64) -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
            failure_threshold,
            timeout_seconds,
            half_open_max_requests: 3,
        }
    }

    pub fn can_attempt(&self, key: &str) -> bool {
        let mut states = self.states.lock().unwrap();
        let state = states
            .entry(key.to_string())
            .or_insert(CircuitBreakerState {
                state: CircuitState::Closed,
                failure_count: 0,
                last_failure_time: None,
                last_success_time: None,
                half_open_request_count: 0,
            });

        match state.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = state.last_failure_time {
                    let elapsed = Utc::now() - last_failure;
                    if elapsed > Duration::seconds(self.timeout_seconds) {
                        state.state = CircuitState::HalfOpen;
                        state.failure_count = 0;
                        state.half_open_request_count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => {
                if state.half_open_request_count >= self.half_open_max_requests {
                    false
                } else {
                    state.half_open_request_count += 1;
                    true
                }
            }
        }
    }

    pub fn record_success(&self, key: &str) {
        let mut states = self.states.lock().unwrap();
        if let Some(state) = states.get_mut(key) {
            if state.state == CircuitState::HalfOpen {
                state.state = CircuitState::Closed;
            }
            state.failure_count = 0;
            state.half_open_request_count = 0;
            state.last_success_time = Some(Utc::now());
        }
    }

    pub fn record_failure(&self, key: &str) {
        let mut states = self.states.lock().unwrap();
        let state = states
            .entry(key.to_string())
            .or_insert(CircuitBreakerState {
                state: CircuitState::Closed,
                failure_count: 0,
                last_failure_time: None,
                last_success_time: None,
                half_open_request_count: 0,
            });

        state.failure_count += 1;
        state.last_failure_time = Some(Utc::now());

        if state.failure_count >= self.failure_threshold {
            state.state = CircuitState::Open;
            tracing::warn!(
                "Circuit breaker opened for '{}' after {} failures",
                key,
                state.failure_count
            );
        }
    }

    pub fn get_state(&self, key: &str) -> CircuitState {
        let states = self.states.lock().unwrap();
        states
            .get(key)
            .map(|s| s.state.clone())
            .unwrap_or(CircuitState::Closed)
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5, 60)
    }
}

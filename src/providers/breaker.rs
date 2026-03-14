use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Default)]
pub struct BreakerConfig {
    pub failure_threshold: u64,
    pub cooldown_secs: u64,
}

#[derive(Debug, Clone)]
pub struct BreakerState {
    pub failures: u64,
    pub first_failure: Option<Instant>,
    pub last_failure: Option<Instant>,
    pub cooldown_until: Option<Instant>,
    pub total_trips: u64,
    pub total_rejections: u64,
}

impl Default for BreakerState {
    fn default() -> Self {
        Self {
            failures: 0,
            first_failure: None,
            last_failure: None,
            cooldown_until: None,
            total_trips: 0,
            total_rejections: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BreakerRegistry {
    failure_counts: Arc<Mutex<HashMap<String, BreakerState>>>,
    config: Arc<Mutex<BreakerConfig>>,
}

impl BreakerRegistry {
    pub fn new(failure_threshold: u64, cooldown_secs: u64) -> Self {
        Self {
            failure_counts: Arc::new(Mutex::new(HashMap::new())),
            config: Arc::new(Mutex::new(BreakerConfig {
                failure_threshold,
                cooldown_secs,
            })),
        }
    }

    pub fn with_config(config: BreakerConfig) -> Self {
        Self {
            failure_counts: Arc::new(Mutex::new(HashMap::new())),
            config: Arc::new(Mutex::new(config)),
        }
    }

    pub fn increment_failure(&self, provider: &str) {
        let mut counts = self.failure_counts.lock().unwrap();
        let state = counts.entry(provider.to_string()).or_default();
        let now = Instant::now();

        state.failures += 1;
        if state.first_failure.is_none() {
            state.first_failure = Some(now);
        }
        state.last_failure = Some(now);
    }

    pub fn is_open(&self, provider: &str) -> bool {
        let config = self.config.lock().unwrap().clone();
        let mut counts = self.failure_counts.lock().unwrap();
        let now = Instant::now();

        let Some(state) = counts.get_mut(provider) else {
            return false;
        };

        if state.failures >= config.failure_threshold {
            if let Some(until) = state.cooldown_until {
                if now < until {
                    state.total_rejections += 1;
                    return true;
                }
            }

            state.cooldown_until = Some(now + Duration::from_secs(config.cooldown_secs));
            state.total_trips += 1;
            state.total_rejections += 1;
            return true;
        }

        false
    }

    pub fn try_acquire(&self, provider: &str) -> bool {
        if self.is_open(provider) {
            return false;
        }
        true
    }

    pub fn record_success(&self, provider: &str) {
        let mut counts = self.failure_counts.lock().unwrap();
        if let Some(state) = counts.get_mut(provider) {
            state.failures = 0;
            state.cooldown_until = None;
        }
    }

    pub fn record_failure(&self, provider: &str) {
        self.increment_failure(provider);
    }

    pub fn reset(&self, provider: &str) {
        let mut counts = self.failure_counts.lock().unwrap();
        counts.remove(provider);
    }

    pub fn reset_all(&self) {
        let mut counts = self.failure_counts.lock().unwrap();
        counts.clear();
    }

    pub fn get_state(&self, provider: &str) -> Option<BreakerState> {
        let counts = self.failure_counts.lock().unwrap();
        counts.get(provider).cloned()
    }

    pub fn get_all_states(&self) -> HashMap<String, BreakerState> {
        let counts = self.failure_counts.lock().unwrap();
        counts.clone()
    }

    pub fn get_config(&self) -> BreakerConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn set_threshold(&self, threshold: u64) {
        let mut config = self.config.lock().unwrap();
        config.failure_threshold = threshold;
    }

    pub fn set_cooldown(&self, secs: u64) {
        let mut config = self.config.lock().unwrap();
        config.cooldown_secs = secs;
    }
}

pub static BREAKER_REGISTRY: LazyLock<Arc<RwLock<BreakerRegistry>>> = LazyLock::new(|| {
    let registry = BreakerRegistry::new(5, 30);
    Arc::new(RwLock::new(registry))
});

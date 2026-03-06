use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt::Debug;
use crate::app::request_redraw;

/// A reactive state container that notifies listeners on change.
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    version: Arc<AtomicU64>,
}

impl<T: Clone + Debug> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
            version: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }

    pub fn version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }

    pub fn set(&self, new_value: T) {
        let mut val = self.value.write().unwrap();
        *val = new_value;
        self.version.fetch_add(1, Ordering::SeqCst);
        log::trace!("Signal updated (v{}): {:?}", self.version(), *val);
        request_redraw();
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut val = self.value.write().unwrap();
        f(&mut val);
        self.version.fetch_add(1, Ordering::SeqCst);
        log::trace!("Signal mutated (v{}): {:?}", self.version(), *val);
        request_redraw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_update() {
        let s = Signal::new(10);
        assert_eq!(s.get(), 10);
        s.set(20);
        assert_eq!(s.get(), 20);
        s.update(|v| *v += 5);
        assert_eq!(s.get(), 25);
    }

    #[test]
    fn test_memo_derivation() {
        let s = Signal::new(10);
        let m = Memo::new(s.clone(), |v| v * 2);
        assert_eq!(m.get(), 20);
        
        s.set(20);
        assert_eq!(m.get(), 40); // Memo must update
    }

    #[test]
    fn test_memo_caching() {
        let s = Signal::new(10);
        let counter = Arc::new(AtomicU64::new(0));
        
        let m = Memo::new(s.clone(), {
            let counter = counter.clone();
            move |v| {
                counter.fetch_add(1, Ordering::SeqCst);
                v + 1
            }
        });

        assert_eq!(m.get(), 11);
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Computed once

        assert_eq!(m.get(), 11);
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Still 1 (Cached)

        s.set(20);
        assert_eq!(m.get(), 21);
        assert_eq!(counter.load(Ordering::SeqCst), 2); // Recomputed
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            version: Arc::clone(&self.version),
        }
    }
}

/// A derived state that caches its value until dependencies change.
/// This prevents unnecessary recalculations and memory pressure (Memoization).
pub struct Memo<T, D> {
    source: Signal<D>,
    compute: Box<dyn Fn(D) -> T>,
    cached_value: Arc<RwLock<T>>,
    last_version: Arc<AtomicU64>,
}

impl<T: Clone + Debug, D: Clone + Debug> Memo<T, D> {
    /// Creates a new derived state (Memo).
    pub fn new(source: Signal<D>, compute: impl Fn(D) -> T + 'static) -> Self {
        let initial_val = compute(source.get());
        let initial_ver = source.version();
        
        Self {
            source,
            compute: Box::new(compute),
            cached_value: Arc::new(RwLock::new(initial_val)),
            last_version: Arc::new(AtomicU64::new(initial_ver)),
        }
    }

    /// Returns the cached value, or recalculates it if the source has changed.
    pub fn get(&self) -> T {
        let current_ver = self.source.version();
        let last_ver = self.last_version.load(Ordering::SeqCst);

        if current_ver > last_ver {
            // Cache is dirty, recalculate
            let mut cache = self.cached_value.write().unwrap();
            *cache = (self.compute)(self.source.get());
            self.last_version.store(current_ver, Ordering::SeqCst);
            log::trace!("Memo recalculated (v{}): {:?}", current_ver, *cache);
            cache.clone()
        } else {
            // Cache is fresh, return immediately
            self.cached_value.read().unwrap().clone()
        }
    }
}

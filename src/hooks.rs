//! Custom hooks for automatic data refreshing and reactive state management
//!
//! This module provides hooks to automatically refresh data at regular intervals,
//! making the UI feel more dynamic and responsive without manual page refreshes.

use dioxus::prelude::*;

/// A custom hook that automatically refreshes data at specified intervals
///
/// This hook creates a periodic polling mechanism that automatically re-executes
/// a data fetching function at regular intervals.
///
/// # Arguments
/// * `interval_ms` - The interval in milliseconds between automatic refreshes
/// * `fetcher` - A closure that returns an async block to fetch data
///
/// # Example
/// ```rust,no_run
/// use crate::hooks::use_auto_refresh;
/// 
/// let mut data = use_signal(|| Vec::new());
/// 
/// use_auto_refresh(2000, || async move {
///     if let Ok(new_data) = fetch_data().await {
///         data.set(new_data);
///     }
/// });
/// ```
pub fn use_auto_refresh<F, Fut>(interval_ms: u64, mut fetcher: F)
where
    F: FnMut() -> Fut + 'static,
    Fut: Future<Output = ()> + 'static,
{
    use_effect(move || {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));
        let mut fetcher = fetcher;
        
        let task = spawn(async move {
            loop {
                interval.tick().await;
                fetcher().await;
            }
        });
        
        // Cleanup: abort the task when the effect is cleaned up
        on_cleanup(move || {
            task.abort();
        });
    });
}

/// Hook to automatically refresh resource data
///
/// This is a higher-level wrapper around use_auto_refresh that works with
/// Dioxus's use_resource pattern.
///
/// # Arguments
/// * `interval_ms` - The interval in milliseconds between refreshes
/// * `key` - A value that, when changed, triggers a refresh
///
/// # Returns
/// * A trigger function that can be called to manually refresh
pub fn use_auto_refresh_resource<T>(
    interval_ms: u64,
    key: T,
) -> impl Fn()
where
    T: Clone + PartialEq + 'static,
{
    let refresh_counter = use_signal(|| 0);

    use_auto_refresh(interval_ms, {
        let refresh_counter = refresh_counter;
        let key = key.clone();
        move || {
            let refresh_counter = refresh_counter;
            let key = key.clone();
            async move {
                // Trigger refresh by incrementing counter
                refresh_counter.set(refresh_counter.read() + 1);
            }
        }
    });

    move || {
        refresh_counter.set(refresh_counter.read() + 1);
    }
}

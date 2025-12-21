use std::fmt::Debug;
use std::sync::{Arc, RwLock};

type Observer<E> = Arc<dyn Fn(&E) + Send + Sync>;

/// A simple event dispatcher that notifies registered observers when events occur.
///
/// # Examples
///
/// ```
/// use education_platform_common::{DateTime, DomainEventDispatcher};
///
/// struct UserCreated {
///     user_id: u64,
///     occurred_at: DateTime,
/// }
///
/// let dispatcher: DomainEventDispatcher<UserCreated> = DomainEventDispatcher::new();
///
/// dispatcher.subscribe(|event| {
///     println!("User {} was created", event.user_id);
/// });
///
/// let event = UserCreated {
///     user_id: 42,
///     occurred_at: DateTime::now(),
/// };
///
/// dispatcher.notify(&event);
/// ```
pub struct DomainEventDispatcher<E> {
    observers: RwLock<Vec<Observer<E>>>,
}

impl<E> Debug for DomainEventDispatcher<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.observer_count();
        f.debug_struct("DomainEventDispatcher")
            .field("observer_count", &count)
            .finish()
    }
}

impl<E> Default for DomainEventDispatcher<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> DomainEventDispatcher<E> {
    /// Creates a new empty event dispatcher.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            observers: RwLock::new(Vec::new()),
        }
    }

    /// Subscribes an observer callback to receive events.
    pub fn subscribe<F>(&self, observer: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let mut observers = self.observers.write().unwrap_or_else(|e| e.into_inner());
        observers.push(Arc::new(observer));
    }

    /// Notifies all registered observers about an event.
    pub fn notify(&self, event: &E) {
        let observers = self.observers.read().unwrap_or_else(|e| e.into_inner());
        for observer in observers.iter() {
            observer(event);
        }
    }

    /// Returns the number of registered observers.
    #[inline]
    #[must_use]
    pub fn observer_count(&self) -> usize {
        self.observers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .len()
    }
}

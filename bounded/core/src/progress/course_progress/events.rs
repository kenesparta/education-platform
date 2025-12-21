use super::CourseProgress;
use education_platform_common::{Email, Id};

/// Event emitted when a user completes a course.
///
/// # Examples
///
/// ```
/// use education_platform_core::CourseEnded;
/// use education_platform_common::{Email, Id};
///
/// let event = CourseEnded::new(
///     Email::new("user@example.com".to_string()).unwrap(),
///     Id::new(),
/// );
///
/// assert_eq!(event.user_email().address(), "user@example.com");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseEnded {
    user_email: Email,
    course_id: Id,
}

impl CourseEnded {
    /// Creates a new `CourseEnded` event.
    #[inline]
    #[must_use]
    pub const fn new(user_email: Email, course_id: Id) -> Self {
        Self {
            user_email,
            course_id,
        }
    }

    /// Returns the email of the user who completed the course.
    #[inline]
    #[must_use]
    pub const fn user_email(&self) -> &Email {
        &self.user_email
    }

    /// Returns the ID of the completed course.
    #[inline]
    #[must_use]
    pub const fn course_id(&self) -> Id {
        self.course_id
    }
}

impl CourseProgress {
    /// Publishes a course ended event to all registered observers.
    pub fn publish_ended(&self) {
        let event = CourseEnded::new(self.user_email.clone(), self.id);
        self.event_dispatcher.notify(&event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LessonProgress;
    use education_platform_common::{DomainEventDispatcher, Entity};
    use std::sync::{Arc, Mutex};

    fn create_test_lesson(name: &str) -> LessonProgress {
        LessonProgress::new(name.to_string(), 1800, None, None).unwrap()
    }

    mod course_ended {
        use super::*;

        #[test]
        fn test_new_creates_event() {
            let email = Email::new("user@example.com".to_string()).unwrap();
            let id = Id::new();

            let event = CourseEnded::new(email.clone(), id);

            assert_eq!(event.user_email().address(), "user@example.com");
            assert_eq!(event.course_id(), id);
        }

        #[test]
        fn test_event_equality() {
            let email = Email::new("user@example.com".to_string()).unwrap();
            let id = Id::new();

            let event1 = CourseEnded::new(email.clone(), id);
            let event2 = CourseEnded::new(email, id);

            assert_eq!(event1, event2);
        }
    }

    mod publish_ended {
        use super::*;

        #[test]
        fn test_publish_ended_notifies_observers() {
            let lesson = create_test_lesson("Intro");
            let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());

            let received_events = Arc::new(Mutex::new(Vec::new()));
            let events_clone = Arc::clone(&received_events);

            dispatcher.subscribe(move |event: &CourseEnded| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let progress = CourseProgress::builder()
                .course_name("Test Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(dispatcher)
                .build()
                .unwrap();

            progress.publish_ended();

            let events = received_events.lock().unwrap();
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].user_email().address(), "user@example.com");
            assert_eq!(events[0].course_id(), progress.id());
        }
    }
}

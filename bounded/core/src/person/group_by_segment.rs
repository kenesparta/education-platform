use crate::Person;
use education_platform_common::Segment;
use std::collections::HashMap;

/// Service for grouping persons by their DNI segment classification.
///
/// This service provides domain logic for organizing persons based on their
/// DNI segment, which reflects the era and conditions of document issuance.
pub struct GroupBySegment;

impl GroupBySegment {
    /// Groups a collection of persons by their DNI segment.
    ///
    /// Persons are organized into a HashMap where keys are DNI segments
    /// (OlderThan1996, NoYellowDNI, WithYellowDNI) and values are vectors
    /// of persons belonging to that segment.
    ///
    /// Uses a functional approach with `fold` to aggregate persons into their
    /// respective segment groups efficiently.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Person, GroupBySegment};
    /// use education_platform_common::Segment;
    ///
    /// let person1 = Person::new(
    ///     "John".to_string(),
    ///     None,
    ///     "Doe".to_string(),
    ///     None,
    ///     "12345678-1".to_string()  // Segment: OlderThan1996
    /// ).unwrap();
    ///
    /// let person2 = Person::new(
    ///     "Jane".to_string(),
    ///     None,
    ///     "Smith".to_string(),
    ///     None,
    ///     "70000001-5".to_string()  // Segment: WithYellowDNI
    /// ).unwrap();
    ///
    /// let persons = vec![person1, person2];
    /// let grouped = GroupBySegment::group_by_segment(persons);
    ///
    /// assert_eq!(grouped.get(&Segment::OlderThan1996).unwrap().len(), 1);
    /// assert_eq!(grouped.get(&Segment::WithYellowDNI).unwrap().len(), 1);
    /// ```
    #[must_use]
    pub fn group_by_segment(person_array: Vec<Person>) -> HashMap<Segment, Vec<Person>> {
        person_array
            .into_iter()
            .fold(HashMap::new(), |mut grouped, person| {
                grouped
                    .entry(person.document().segment().clone())
                    .or_default()
                    .push(person);
                grouped
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use education_platform_common::Entity;

    #[test]
    fn test_group_by_segment_with_multiple_segments() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(), // OlderThan1996
        )
        .unwrap();

        let person2 = Person::new(
            "Jane".to_string(),
            None,
            "Smith".to_string(),
            None,
            "70000001-5".to_string(), // WithYellowDNI
        )
        .unwrap();

        let person3 = Person::new(
            "Bob".to_string(),
            None,
            "Johnson".to_string(),
            None,
            "40000001-3".to_string(), // NoYellowDNI
        )
        .unwrap();

        let persons = vec![person1, person2, person3];
        let grouped = GroupBySegment::group_by_segment(persons);

        assert_eq!(grouped.len(), 3);
        assert_eq!(grouped.get(&Segment::OlderThan1996).unwrap().len(), 1);
        assert_eq!(grouped.get(&Segment::WithYellowDNI).unwrap().len(), 1);
        assert_eq!(grouped.get(&Segment::NoYellowDNI).unwrap().len(), 1);
    }

    #[test]
    fn test_group_by_segment_with_empty_array() {
        let persons: Vec<Person> = vec![];
        let grouped = GroupBySegment::group_by_segment(persons);

        assert_eq!(grouped.len(), 0);
    }

    #[test]
    fn test_group_by_segment_all_same_segment() {
        let person1 = Person::new(
            "Alice".to_string(),
            None,
            "Brown".to_string(),
            None,
            "12345678-1".to_string(), // OlderThan1996
        )
        .unwrap();

        let person2 = Person::new(
            "Charlie".to_string(),
            None,
            "Davis".to_string(),
            None,
            "00000001-4".to_string(), // OlderThan1996
        )
        .unwrap();

        let person3 = Person::new(
            "Eve".to_string(),
            None,
            "Wilson".to_string(),
            None,
            "20000001-9".to_string(), // OlderThan1996
        )
        .unwrap();

        let persons = vec![person1, person2, person3];
        let grouped = GroupBySegment::group_by_segment(persons);

        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get(&Segment::OlderThan1996).unwrap().len(), 3);
    }

    #[test]
    fn test_group_by_segment_multiple_persons_per_segment() {
        let older1 = Person::new(
            "María".to_string(),
            None,
            "García".to_string(),
            None,
            "12345678-1".to_string(), // OlderThan1996
        )
        .unwrap();

        let older2 = Person::new(
            "Carlos".to_string(),
            None,
            "López".to_string(),
            None,
            "00000001-4".to_string(), // OlderThan1996
        )
        .unwrap();

        let yellow1 = Person::new(
            "Ana".to_string(),
            None,
            "Martínez".to_string(),
            None,
            "70000001-5".to_string(), // WithYellowDNI
        )
        .unwrap();

        let yellow2 = Person::new(
            "Luis".to_string(),
            None,
            "Rodríguez".to_string(),
            None,
            "80000001-2".to_string(), // WithYellowDNI
        )
        .unwrap();

        let no_yellow = Person::new(
            "Pedro".to_string(),
            None,
            "Sánchez".to_string(),
            None,
            "50000001-1".to_string(), // NoYellowDNI
        )
        .unwrap();

        let persons = vec![older1, older2, yellow1, yellow2, no_yellow];
        let grouped = GroupBySegment::group_by_segment(persons);

        assert_eq!(grouped.len(), 3);
        assert_eq!(grouped.get(&Segment::OlderThan1996).unwrap().len(), 2);
        assert_eq!(grouped.get(&Segment::WithYellowDNI).unwrap().len(), 2);
        assert_eq!(grouped.get(&Segment::NoYellowDNI).unwrap().len(), 1);
    }

    #[test]
    fn test_group_by_segment_preserves_person_data() {
        let person1 = Person::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person1_id = person1.id();
        let persons = vec![person1];
        let grouped = GroupBySegment::group_by_segment(persons);

        let older_persons = grouped.get(&Segment::OlderThan1996).unwrap();
        assert_eq!(older_persons.len(), 1);
        assert_eq!(older_persons[0].id(), person1_id);
        assert_eq!(older_persons[0].name().first_name(), "John");
        assert_eq!(older_persons[0].name().middle_name(), Some("Michael"));
        assert_eq!(older_persons[0].name().last_name(), "Doe");
        assert_eq!(older_persons[0].document().value(), "12345678");
    }

    #[test]
    fn test_group_by_segment_order_independence() {
        let person1 = Person::new(
            "Ana".to_string(),
            None,
            "Torres".to_string(),
            None,
            "70000001-5".to_string(),
        )
        .unwrap();

        let person2 = Person::new(
            "José".to_string(),
            None,
            "Ramírez".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let grouped1 = GroupBySegment::group_by_segment(vec![person1.clone(), person2.clone()]);
        let grouped2 = GroupBySegment::group_by_segment(vec![person2, person1]);

        assert_eq!(grouped1.len(), grouped2.len());
        assert_eq!(
            grouped1.get(&Segment::OlderThan1996).unwrap().len(),
            grouped2.get(&Segment::OlderThan1996).unwrap().len()
        );
        assert_eq!(
            grouped1.get(&Segment::WithYellowDNI).unwrap().len(),
            grouped2.get(&Segment::WithYellowDNI).unwrap().len()
        );
    }

    #[test]
    fn test_group_by_segment_with_all_segment_types() {
        let older = Person::new(
            "Miguel".to_string(),
            None,
            "Fernández".to_string(),
            None,
            "00000001-4".to_string(), // OlderThan1996 (starts with 0)
        )
        .unwrap();

        let no_yellow4 = Person::new(
            "Laura".to_string(),
            None,
            "González".to_string(),
            None,
            "40000001-3".to_string(), // NoYellowDNI (starts with 4)
        )
        .unwrap();

        let no_yellow5 = Person::new(
            "Diego".to_string(),
            None,
            "Jiménez".to_string(),
            None,
            "50000001-1".to_string(), // NoYellowDNI (starts with 5)
        )
        .unwrap();

        let with_yellow = Person::new(
            "Isabel".to_string(),
            None,
            "Morales".to_string(),
            None,
            "90000001-0".to_string(), // WithYellowDNI (starts with 9)
        )
        .unwrap();

        let persons = vec![older, no_yellow4, no_yellow5, with_yellow];
        let grouped = GroupBySegment::group_by_segment(persons);

        assert_eq!(grouped.len(), 3);
        assert!(grouped.contains_key(&Segment::OlderThan1996));
        assert!(grouped.contains_key(&Segment::NoYellowDNI));
        assert!(grouped.contains_key(&Segment::WithYellowDNI));
        assert_eq!(grouped.get(&Segment::NoYellowDNI).unwrap().len(), 2);
    }

    #[test]
    fn test_group_by_segment_result_is_owned() {
        let person = Person::new(
            "Test".to_string(),
            None,
            "Person".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let persons = vec![person];
        let grouped = GroupBySegment::group_by_segment(persons);

        // HashMap should own the persons
        let older_persons = grouped.get(&Segment::OlderThan1996).unwrap();
        assert_eq!(older_persons.len(), 1);
    }

    #[test]
    fn test_group_by_segment_with_duplicate_persons() {
        let person1 = Person::new(
            "Same".to_string(),
            None,
            "Name".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = Person::new(
            "Same".to_string(),
            None,
            "Name".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        // Different persons (different IDs) with same name and DNI
        assert_ne!(person1.id(), person2.id());

        let persons = vec![person1, person2];
        let grouped = GroupBySegment::group_by_segment(persons);

        // Both should be grouped together
        assert_eq!(grouped.get(&Segment::OlderThan1996).unwrap().len(), 2);
    }
}

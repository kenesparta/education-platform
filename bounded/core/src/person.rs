use education_platform_common::{Dni, DniError, Entity, Id, IdError, PersonName, PersonNameError};
use thiserror::Error;

/// Error types for Person validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PersonError {
    #[error("ID not valid: {0}")]
    IdError(#[from] IdError),

    #[error("Person name is not valid: {0}")]
    PersonNameError(#[from] PersonNameError),

    #[error("DNI not valid: {0}")]
    DniError(#[from] DniError),
}

/// A person entity with a unique identity.
///
/// This is a DDD Entity - two persons with different IDs are considered different
/// even if all other attributes match. Equality is based solely on the `id` field.
///
/// # Examples
///
/// ```
/// use education_platform_core::person::Person;
///
/// let person = Person::new(
///     "John".to_string(),
///     None,
///     "Doe".to_string(),
///     None,
///     "12345678-1".to_string()
/// ).unwrap();
///
/// assert_eq!(person.name().full_name(), "John Doe");
/// assert_eq!(person.document().value(), "12345678");
/// ```
#[derive(Debug, Clone)]
pub struct Person {
    id: Id,
    name: PersonName,
    document: Dni,
}

impl Person {
    /// Creates a new Person entity with a unique ID.
    ///
    /// # Errors
    ///
    /// Returns error if the name components are invalid or the DNI is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::person::Person;
    ///
    /// let person = Person::new(
    ///     "María".to_string(),
    ///     None,
    ///     "García".to_string(),
    ///     Some("Rodríguez".to_string()),
    ///     "17801146-0".to_string()
    /// ).unwrap();
    ///
    /// assert_eq!(person.name().first_name(), "María");
    /// assert_eq!(person.name().last_name(), "García");
    ///
    /// // Invalid DNI returns error
    /// let invalid = Person::new(
    ///     "John".to_string(),
    ///     None,
    ///     "Doe".to_string(),
    ///     None,
    ///     "invalid".to_string()
    /// );
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        second_last_name: Option<String>,
        document: String,
    ) -> Result<Self, PersonError> {
        Ok(Self {
            id: Id::new(),
            name: PersonName::new(first_name, middle_name, last_name, second_last_name)?,
            document: Dni::new(document)?,
        })
    }

    /// Returns a reference to the person's name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::person::Person;
    ///
    /// let person = Person::new(
    ///     "John".to_string(),
    ///     Some("Michael".to_string()),
    ///     "Doe".to_string(),
    ///     None,
    ///     "12345678-1".to_string()
    /// ).unwrap();
    ///
    /// assert_eq!(person.name().full_name(), "John Michael Doe");
    /// ```
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &PersonName {
        &self.name
    }

    /// Returns a reference to the person's document (DNI).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::person::Person;
    ///
    /// let person = Person::new(
    ///     "John".to_string(),
    ///     None,
    ///     "Doe".to_string(),
    ///     None,
    ///     "12345678-1".to_string()
    /// ).unwrap();
    ///
    /// assert_eq!(person.document().value(), "12345678");
    /// assert_eq!(person.document().verification_char(), "1");
    /// ```
    #[inline]
    #[must_use]
    pub const fn document(&self) -> &Dni {
        &self.document
    }
}

// Implement Entity trait (composition over inheritance)
impl Entity for Person {
    fn id(&self) -> Id {
        self.id
    }
}

// Entity equality based on ID only (DDD principle)
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_person_with_valid_data() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        );

        assert!(person.is_ok());
        let person = person.unwrap();
        assert_eq!(person.name().first_name(), "John");
        assert_eq!(person.name().last_name(), "Doe");
        assert_eq!(person.document().value(), "12345678");
    }

    #[test]
    fn test_new_with_all_name_components() {
        let person = Person::new(
            "María".to_string(),
            Some("Isabel".to_string()),
            "García".to_string(),
            Some("Rodríguez".to_string()),
            "17801146-0".to_string(),
        );

        assert!(person.is_ok());
        let person = person.unwrap();
        assert_eq!(person.name().full_name(), "María Isabel García Rodríguez");
        assert_eq!(person.document().value(), "17801146");
    }

    #[test]
    fn test_new_with_invalid_name_returns_error() {
        let result = Person::new(
            "".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        );

        assert!(result.is_err());
        assert!(matches!(result, Err(PersonError::PersonNameError(_))));
    }

    #[test]
    fn test_new_with_invalid_dni_returns_error() {
        let result = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "invalid-dni".to_string(),
        );

        assert!(result.is_err());
        assert!(matches!(result, Err(PersonError::DniError(_))));
    }

    #[test]
    fn test_id_returns_unique_identifier() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let id = person.id();
        assert_eq!(id.to_string().len(), 26);
    }

    #[test]
    fn test_name_getter_returns_correct_name() {
        let person = Person::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        assert_eq!(person.name().first_name(), "John");
        assert_eq!(person.name().middle_name(), Some("Michael"));
        assert_eq!(person.name().last_name(), "Doe");
    }

    #[test]
    fn test_document_getter_returns_correct_dni() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "17801146-0".to_string(),
        )
        .unwrap();

        assert_eq!(person.document().value(), "17801146");
        assert_eq!(person.document().verification_char(), "0");
    }

    #[test]
    fn test_clone_creates_equal_instance() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let cloned = person.clone();

        assert_eq!(person, cloned);
        assert_eq!(person.id(), cloned.id());
        assert_eq!(person.name().full_name(), cloned.name().full_name());
        assert_eq!(
            person.document().with_verification_char(),
            cloned.document().with_verification_char()
        );
    }

    #[test]
    fn test_clone_is_independent() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = person1.clone();

        // They should be equal (same ID)
        assert_eq!(person1, person2);

        // But modifying one doesn't affect the other (test independence)
        // Since Person fields are private and immutable; we verify they're separate instances
        assert_eq!(person1.id(), person2.id());
    }

    #[test]
    fn test_eq_compares_by_id_only() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = Person::new(
            "Jane".to_string(),
            None,
            "Smith".to_string(),
            None,
            "87654321-0".to_string(),
        )
        .unwrap();

        // Different IDs means not equal, even if we manually create with same data
        assert_ne!(person1, person2);
    }

    #[test]
    fn test_eq_same_id_means_equal() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let cloned = person.clone();

        // Same ID (from clone) means equal
        assert_eq!(person, cloned);
    }

    #[test]
    fn test_eq_reflexive() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        assert_eq!(person, person);
    }

    #[test]
    fn test_eq_symmetric() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = person1.clone();

        assert_eq!(person1, person2);
        assert_eq!(person2, person1);
    }

    #[test]
    fn test_eq_transitive() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = person1.clone();
        let person3 = person1.clone();

        assert_eq!(person1, person2);
        assert_eq!(person2, person3);
        assert_eq!(person1, person3);
    }

    #[test]
    fn test_entity_semantics_different_data_same_id() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let cloned = person.clone();

        // Even though they have the same ID, they represent the same entity
        assert_eq!(person.id(), cloned.id());
        assert_eq!(person, cloned);
    }

    #[test]
    fn test_different_persons_have_different_ids() {
        let person1 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let person2 = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        // Same data but different IDs (each new() creates new ID)
        assert_ne!(person1.id(), person2.id());
        assert_ne!(person1, person2);
    }

    #[test]
    fn test_debug_format_includes_all_fields() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let debug_output = format!("{:?}", person);
        assert!(debug_output.contains("Person"));
        assert!(debug_output.contains("id"));
        assert!(debug_output.contains("name"));
        assert!(debug_output.contains("document"));
    }

    #[test]
    fn test_trimming_of_name_components() {
        let person = Person::new(
            "  John  ".to_string(),
            Some("  Michael  ".to_string()),
            "  Doe  ".to_string(),
            None,
            "  12345678-1  ".to_string(),
        )
        .unwrap();

        assert_eq!(person.name().first_name(), "John");
        assert_eq!(person.name().middle_name(), Some("Michael"));
        assert_eq!(person.name().last_name(), "Doe");
        assert_eq!(person.document().value(), "12345678");
    }

    #[test]
    fn test_spanish_name_with_special_characters() {
        let person = Person::new(
            "José".to_string(),
            None,
            "Martínez".to_string(),
            Some("Fernández".to_string()),
            "00000001-I".to_string(),
        )
        .unwrap();

        assert_eq!(person.name().full_name(), "José Martínez Fernández");
        assert_eq!(person.document().verification_char(), "I");
    }

    #[test]
    fn test_entity_trait_implementation() {
        let person = Person::new(
            "John".to_string(),
            None,
            "Doe".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        // Can call id() through Entity trait
        let id_from_trait: Id = Entity::id(&person);
        let id_from_method = person.id();

        assert_eq!(id_from_trait, id_from_method);
    }

    #[test]
    fn test_entity_trait_polymorphism() {
        let person = Person::new(
            "Alice".to_string(),
            None,
            "Smith".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        // Can use Person as Entity trait object
        fn get_entity_id(entity: &dyn Entity) -> Id {
            entity.id()
        }

        let id = get_entity_id(&person);
        assert_eq!(id, person.id());
    }

    #[test]
    fn test_entity_trait_with_generics() {
        fn print_entity_id<T: Entity>(entity: &T) -> Id {
            entity.id()
        }

        let person = Person::new(
            "Bob".to_string(),
            None,
            "Johnson".to_string(),
            None,
            "87654321-0".to_string(),
        )
        .unwrap();

        let id = print_entity_id(&person);
        assert_eq!(id, person.id());
        assert_eq!(id.to_string().len(), 26);
    }

    #[test]
    fn test_entity_id_never_changes() {
        let person = Person::new(
            "Charlie".to_string(),
            None,
            "Brown".to_string(),
            None,
            "12345678-1".to_string(),
        )
        .unwrap();

        let id1 = person.id();
        let id2 = person.id();
        let id3 = Entity::id(&person);

        // ID should be consistent across calls
        assert_eq!(id1, id2);
        assert_eq!(id2, id3);
    }
}

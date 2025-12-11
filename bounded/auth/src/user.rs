use education_platform_common::{
    ArgonVariant, Dni, DniError, Email, EmailError, Entity, HashedPassword, HashedPasswordError,
    HashingAlgorithm, Id, IdError, PersonName, PersonNameError,
};
use thiserror::Error;

/// Error types for User validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names)]
pub enum UserError {
    #[error("ID not valid: {0}")]
    IdError(#[from] IdError),

    #[error("Person name is not valid: {0}")]
    PersonNameError(#[from] PersonNameError),

    #[error("DNI not valid: {0}")]
    DniError(#[from] DniError),

    #[error("Email not valid: {0}")]
    EmailError(#[from] EmailError),

    #[error("Password doesn't match the security requirements: {0}")]
    HashedPasswordError(#[from] HashedPasswordError),
}

/// Represents a user entity in the authentication bounded context.
///
/// A `User` is an entity (not a value object) with unique identity that persists
/// over time. Users are identified by their `Id` and contain validated personal
/// information including name, document (DNI), email, and optional password hash.
///
/// # Domain-Driven Design
///
/// This is an **Entity** because:
/// - Has unique identity (`Id`) that persists over time
/// - Equality is based on identity, not attributes
/// - Has a lifecycle and can be tracked across operations
/// - Attributes can change while maintaining the same identity
///
/// # Examples
///
/// ```
/// use education_platform_auth::User;
///
/// let user = User::new(
///     "John".to_string(),
///     None,
///     "Doe".to_string(),
///     None,
///     "12345678-1".to_string(),
///     "john.doe@example.com".to_string(),
///     Some("$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG".to_string()),
/// ).unwrap();
///
/// assert_eq!(user.name().full_name(), "John Doe");
/// assert_eq!(user.email().address(), "john.doe@example.com");
/// ```
#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    name: PersonName,
    document: Dni,
    email: Email,
    password: Option<HashedPassword>,
}

impl User {
    /// Creates a new `User` entity with validated components.
    ///
    /// Generates a new unique `Id` for the user and validates all input fields.
    /// The password, if provided, must be a properly formatted hash string for
    /// the Argon2id algorithm.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Name components are invalid (empty, too long, invalid characters)
    /// - Document (DNI) format is invalid
    /// - Email format is invalid
    /// - Password hash format is invalid (if provided)
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// // Create user with password
    /// let user = User::new(
    ///     "Jane".to_string(),
    ///     Some("Marie".to_string()),
    ///     "Smith".to_string(),
    ///     None,
    ///     "00000001-I".to_string(),
    ///     "jane.smith@example.com".to_string(),
    ///     Some("$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG".to_string()),
    /// );
    /// assert!(user.is_ok());
    ///
    /// // Create user without password (e.g., OAuth user)
    /// let oauth_user = User::new(
    ///     "Bob".to_string(),
    ///     None,
    ///     "Johnson".to_string(),
    ///     None,
    ///     "17801146-0".to_string(),
    ///     "bob@example.com".to_string(),
    ///     None,
    /// );
    /// assert!(oauth_user.is_ok());
    ///
    /// // Invalid email
    /// let invalid = User::new(
    ///     "Test".to_string(),
    ///     None,
    ///     "User".to_string(),
    ///     None,
    ///     "12345678-1".to_string(),
    ///     "invalid-email".to_string(),
    ///     None,
    /// );
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        second_last_name: Option<String>,
        document: String,
        email: String,
        password: Option<String>,
    ) -> Result<Self, UserError> {
        let name = PersonName::new(first_name, middle_name, last_name, second_last_name)?;
        let document = Dni::new(document)?;
        let email = Email::new(email)?;
        let password = password
            .map(|p| HashedPassword::new(p, HashingAlgorithm::Argon(ArgonVariant::Argon2id)))
            .transpose()?;
        let id = Id::default();

        Ok(Self {
            id,
            name,
            document,
            email,
            password,
        })
    }

    /// Returns a reference to the user's full name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// let user = User::new(
    ///     "Alice".to_string(),
    ///     None,
    ///     "Wonder".to_string(),
    ///     None,
    ///     "12345678-1".to_string(),
    ///     "alice@example.com".to_string(),
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(user.name().full_name(), "Alice Wonder");
    /// ```
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &PersonName {
        &self.name
    }

    /// Returns a reference to the user's document (DNI).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// let user = User::new(
    ///     "Bob".to_string(),
    ///     None,
    ///     "Builder".to_string(),
    ///     None,
    ///     "00000001-I".to_string(),
    ///     "bob@example.com".to_string(),
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(user.document().with_verification_char(), "00000001-I");
    /// assert_eq!(user.document().value(), "00000001");
    /// ```
    #[inline]
    #[must_use]
    pub const fn document(&self) -> &Dni {
        &self.document
    }

    /// Returns a reference to the user's email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// let user = User::new(
    ///     "Charlie".to_string(),
    ///     None,
    ///     "Brown".to_string(),
    ///     None,
    ///     "17801146-0".to_string(),
    ///     "charlie@example.com".to_string(),
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(user.email().address(), "charlie@example.com");
    /// assert_eq!(user.email().domain().unwrap(), "example.com");
    /// ```
    #[inline]
    #[must_use]
    pub const fn email(&self) -> &Email {
        &self.email
    }

    /// Returns a reference to the user's hashed password, if set.
    ///
    /// Returns `None` for users authenticated via external providers (OAuth, SSO, etc.)
    /// who don't have a password stored in the system.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// // User with password
    /// let user_with_pwd = User::new(
    ///     "Dave".to_string(),
    ///     None,
    ///     "Davis".to_string(),
    ///     None,
    ///     "00000001-4".to_string(),
    ///     "dave@example.com".to_string(),
    ///     Some("$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG".to_string()),
    /// ).unwrap();
    /// assert!(user_with_pwd.password().is_some());
    ///
    /// // OAuth user without password
    /// let oauth_user = User::new(
    ///     "Eve".to_string(),
    ///     None,
    ///     "Evans".to_string(),
    ///     None,
    ///     "12345678-1".to_string(),
    ///     "eve@example.com".to_string(),
    ///     None,
    /// ).unwrap();
    /// assert!(oauth_user.password().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn password(&self) -> Option<&HashedPassword> {
        self.password.as_ref()
    }

    /// Checks if the user has a password set.
    ///
    /// Returns `true` if the user has a password hash stored, `false` otherwise.
    /// Users without passwords typically authenticate via external providers.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_auth::User;
    ///
    /// let user_with_pwd = User::new(
    ///     "Frank".to_string(),
    ///     None,
    ///     "Foster".to_string(),
    ///     None,
    ///     "00000001-4".to_string(),
    ///     "frank@example.com".to_string(),
    ///     Some("$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG".to_string()),
    /// ).unwrap();
    /// assert!(user_with_pwd.has_password());
    ///
    /// let oauth_user = User::new(
    ///     "Grace".to_string(),
    ///     None,
    ///     "Green".to_string(),
    ///     None,
    ///     "00000001-I".to_string(),
    ///     "grace@example.com".to_string(),
    ///     None,
    /// ).unwrap();
    /// assert!(!oauth_user.has_password());
    /// ```
    #[inline]
    #[must_use]
    pub const fn has_password(&self) -> bool {
        self.password.is_some()
    }
}

impl Entity for User {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ARGON2ID_HASH: &str =
        "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";

    mod constructor {
        use super::*;

        #[test]
        fn test_new_with_all_fields_returns_ok() {
            let result = User::new(
                "John".to_string(),
                Some("Michael".to_string()),
                "Doe".to_string(),
                Some("Smith".to_string()),
                "12345678-1".to_string(),
                "john.doe@example.com".to_string(),
                Some(VALID_ARGON2ID_HASH.to_string()),
            );
            assert!(result.is_ok());
        }

        #[test]
        fn test_new_without_middle_name_returns_ok() {
            let result = User::new(
                "Jane".to_string(),
                None,
                "Doe".to_string(),
                None,
                "00000001-I".to_string(),
                "jane@example.com".to_string(),
                None,
            );
            assert!(result.is_ok());
        }

        #[test]
        fn test_new_without_password_returns_ok() {
            let result = User::new(
                "Bob".to_string(),
                None,
                "Johnson".to_string(),
                None,
                "17801146-0".to_string(),
                "bob@example.com".to_string(),
                None,
            );
            assert!(result.is_ok());
            assert!(!result.unwrap().has_password());
        }

        #[test]
        fn test_new_with_invalid_first_name_returns_error() {
            let result = User::new(
                "".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "test@example.com".to_string(),
                None,
            );
            assert!(matches!(result, Err(UserError::PersonNameError(_))));
        }

        #[test]
        fn test_new_with_invalid_last_name_returns_error() {
            let result = User::new(
                "John".to_string(),
                None,
                "".to_string(),
                None,
                "12345678-1".to_string(),
                "test@example.com".to_string(),
                None,
            );
            assert!(matches!(result, Err(UserError::PersonNameError(_))));
        }

        #[test]
        fn test_new_with_invalid_dni_returns_error() {
            let result = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "invalid-dni".to_string(),
                "test@example.com".to_string(),
                None,
            );
            assert!(matches!(result, Err(UserError::DniError(_))));
        }

        #[test]
        fn test_new_with_invalid_email_returns_error() {
            let result = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "invalid-email".to_string(),
                None,
            );
            assert!(matches!(result, Err(UserError::EmailError(_))));
        }

        #[test]
        fn test_new_with_invalid_password_hash_returns_error() {
            let result = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "test@example.com".to_string(),
                Some("invalid-hash".to_string()),
            );
            assert!(matches!(result, Err(UserError::HashedPasswordError(_))));
        }

        #[test]
        fn test_new_trims_whitespace_from_inputs() {
            let result = User::new(
                "  John  ".to_string(),
                None,
                "  Doe  ".to_string(),
                None,
                " 12345678-1 ".to_string(),
                " test@example.com ".to_string(),
                None,
            );
            assert!(result.is_ok());
            let user = result.unwrap();
            assert_eq!(user.name().first_name(), "John");
            assert_eq!(user.name().last_name(), "Doe");
        }

        #[test]
        fn test_new_generates_unique_ids() {
            let user1 = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "john@example.com".to_string(),
                None,
            )
            .unwrap();

            let user2 = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "john@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_ne!(user1.id(), user2.id());
        }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_returns_correct_value() {
            let user = User::new(
                "Alice".to_string(),
                Some("Marie".to_string()),
                "Wonder".to_string(),
                None,
                "12345678-1".to_string(),
                "alice@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_eq!(user.name().full_name(), "Alice Marie Wonder");
            assert_eq!(user.name().first_name(), "Alice");
            assert_eq!(user.name().middle_name(), Some("Marie"));
            assert_eq!(user.name().last_name(), "Wonder");
        }

        #[test]
        fn test_document_returns_correct_value() {
            let user = User::new(
                "Bob".to_string(),
                None,
                "Builder".to_string(),
                None,
                "00000001-I".to_string(),
                "bob@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_eq!(user.document().with_verification_char(), "00000001-I");
            assert_eq!(user.document().value(), "00000001");
        }

        #[test]
        fn test_email_returns_correct_value() {
            let user = User::new(
                "Charlie".to_string(),
                None,
                "Brown".to_string(),
                None,
                "17801146-0".to_string(),
                "charlie@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_eq!(user.email().address(), "charlie@example.com");
            assert_eq!(user.email().local_part().unwrap(), "charlie");
            assert_eq!(user.email().domain().unwrap(), "example.com");
        }

        #[test]
        fn test_password_returns_some_when_set() {
            let user = User::new(
                "Dave".to_string(),
                None,
                "Davis".to_string(),
                None,
                "00000001-4".to_string(),
                "dave@example.com".to_string(),
                Some(VALID_ARGON2ID_HASH.to_string()),
            )
            .unwrap();

            assert!(user.password().is_some());
            assert_eq!(user.password().unwrap().value(), VALID_ARGON2ID_HASH);
        }

        #[test]
        fn test_password_returns_none_when_not_set() {
            let user = User::new(
                "Eve".to_string(),
                None,
                "Evans".to_string(),
                None,
                "12345678-1".to_string(),
                "eve@example.com".to_string(),
                None,
            )
            .unwrap();

            assert!(user.password().is_none());
        }

        #[test]
        fn test_has_password_returns_true_when_set() {
            let user = User::new(
                "Frank".to_string(),
                None,
                "Foster".to_string(),
                None,
                "00000001-4".to_string(),
                "frank@example.com".to_string(),
                Some(VALID_ARGON2ID_HASH.to_string()),
            )
            .unwrap();

            assert!(user.has_password());
        }

        #[test]
        fn test_has_password_returns_false_when_not_set() {
            let user = User::new(
                "Grace".to_string(),
                None,
                "Green".to_string(),
                None,
                "00000001-I".to_string(),
                "grace@example.com".to_string(),
                None,
            )
            .unwrap();

            assert!(!user.has_password());
        }

        #[test]
        fn test_id_returns_unique_identifier() {
            let user = User::new(
                "Henry".to_string(),
                None,
                "Hill".to_string(),
                None,
                "17801146-0".to_string(),
                "henry@example.com".to_string(),
                None,
            )
            .unwrap();

            let id = user.id();
            assert_eq!(id, user.id());
        }
    }

    mod entity_behavior {
        use super::*;

        #[test]
        fn test_equality_based_on_id() {
            let user1 = User::new(
                "John".to_string(),
                None,
                "Doe".to_string(),
                None,
                "12345678-1".to_string(),
                "john@example.com".to_string(),
                None,
            )
            .unwrap();

            let user2 = User::new(
                "Jane".to_string(),
                None,
                "Smith".to_string(),
                None,
                "00000001-I".to_string(),
                "jane@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_ne!(user1, user2);
        }

        #[test]
        fn test_clone_creates_equal_user() {
            let user = User::new(
                "Alice".to_string(),
                None,
                "Wonder".to_string(),
                None,
                "12345678-1".to_string(),
                "alice@example.com".to_string(),
                None,
            )
            .unwrap();

            let cloned = user.clone();
            assert_eq!(user, cloned);
            assert_eq!(user.id(), cloned.id());
        }

        #[test]
        fn test_users_with_same_data_but_different_ids_are_not_equal() {
            let user1 = User::new(
                "Bob".to_string(),
                None,
                "Johnson".to_string(),
                None,
                "17801146-0".to_string(),
                "bob@example.com".to_string(),
                None,
            )
            .unwrap();

            let user2 = User::new(
                "Bob".to_string(),
                None,
                "Johnson".to_string(),
                None,
                "17801146-0".to_string(),
                "bob@example.com".to_string(),
                None,
            )
            .unwrap();

            assert_ne!(user1, user2);
        }

        #[test]
        fn test_entity_trait_implementation() {
            let user = User::new(
                "Charlie".to_string(),
                None,
                "Brown".to_string(),
                None,
                "55555555-E".to_string(),
                "charlie@example.com".to_string(),
                None,
            )
            .unwrap();

            let entity_id = user.id();
            assert_eq!(entity_id, user.id);
        }
    }

    mod real_world_scenarios {
        use super::*;

        #[test]
        fn test_create_standard_user_with_password() {
            let user = User::new(
                "María".to_string(),
                Some("Carmen".to_string()),
                "García".to_string(),
                Some("López".to_string()),
                "12345678-1".to_string(),
                "maria.garcia@example.es".to_string(),
                Some(VALID_ARGON2ID_HASH.to_string()),
            )
            .unwrap();

            assert!(user.has_password());
            assert_eq!(user.name().full_name(), "María Carmen García López");
            assert_eq!(user.email().domain().unwrap(), "example.es");
        }

        #[test]
        fn test_create_oauth_user_without_password() {
            let user = User::new(
                "José".to_string(),
                None,
                "Rodríguez".to_string(),
                None,
                "00000001-I".to_string(),
                "jose@gmail.com".to_string(),
                None,
            )
            .unwrap();

            assert!(!user.has_password());
            assert_eq!(user.name().full_name(), "José Rodríguez");
        }

        #[test]
        fn test_create_user_with_peruvian_dni() {
            let user = User::new(
                "Luis".to_string(),
                None,
                "Pérez".to_string(),
                None,
                "98765432-1".to_string(),
                "luis.perez@outlook.com".to_string(),
                Some(VALID_ARGON2ID_HASH.to_string()),
            )
            .unwrap();

            assert_eq!(user.document().with_verification_char(), "98765432-1");
        }
    }
}

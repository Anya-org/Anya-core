# Hexagonal Architecture in Anya Core

This document describes the Hexagonal (Ports and Adapters) architecture used in Anya Core.

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
- [Directory Structure](#directory-structure)
- [Implementing a New Feature](#implementing-a-new-feature)
- [Testing](#testing)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Overview

Hexagonal Architecture, also known as Ports and Adapters, is an architectural pattern that isolates the core business logic from external concerns. This separation makes the application more maintainable, testable, and adaptable to change.

## Core Concepts

### Domain Layer

- Contains the core business logic
- Pure Rust code with no external dependencies
- Defines domain models and business rules

### Application Layer

- Orchestrates the flow of data between the domain and infrastructure layers
- Implements use cases
- Defines ports (traits) for external interactions

### Infrastructure Layer

- Implements the ports defined by the application layer
- Handles external concerns like:
  - Database access
  - Network communication
  - File I/O
  - External services

## Directory Structure

```
src/
├── domain/               # Domain layer
│   ├── models/          # Domain models
│   └── services/        # Domain services
├── application/         # Application layer
│   ├── ports/           # Ports (traits)
│   └── services/        # Application services
└── infrastructure/      # Infrastructure layer
    ├── adapters/        # Adapters implementing ports
    └── config/          # Configuration
```

## Implementing a New Feature

### 1. Define Domain Models

```rust
// domain/models/user.rs
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
}
```

### 2. Define Ports (Traits)

```rust
// application/ports/user_repository.rs
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, Error>;
    async fn save(&self, user: &User) -> Result<(), Error>;
}
```

### 3. Implement Application Service

```rust
// application/services/user_service.rs
pub struct UserService<T: UserRepository> {
    user_repository: Arc<T>,
}

impl<T: UserRepository> UserService<T> {
    pub async fn get_user(&self, id: &UserId) -> Result<Option<User>, Error> {
        self.user_repository.find_by_id(id).await
    }
}
```

### 4. Implement Adapters

```rust
// infrastructure/adapters/postgres_user_repository.rs
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, Error> {
        // Implementation using SQLx
    }
}
```

## Testing

### Unit Tests

Test domain and application layers in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_get_user() {
        // Setup mock
        let mut mock_repo = MockUserRepository::new();
        mock_repo
            .expect_find_by_id()
            .returning(|_| Ok(Some(User::new("test"))));
            
        // Test
        let service = UserService::new(Arc::new(mock_repo));
        let result = service.get_user(&UserId::new()).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Test the entire stack:

```rust
#[tokio::test]
async fn test_user_flow() {
    // Setup test database
    let pool = setup_test_db().await;
    
    // Create repository with test DB
    let repo = PostgresUserRepository::new(pool);
    
    // Test operations
    let user = User::new("test");
    repo.save(&user).await.unwrap();
    
    let found = repo.find_by_id(&user.id).await.unwrap();
    assert!(found.is_some());
}
```

## Best Practices

### 1. Dependency Rule

Dependencies should always point inward:

- Domain has no dependencies
- Application depends on domain
- Infrastructure depends on application and domain

### 2. Use Traits for Dependencies

```rust
// Good: Depend on trait
struct UserService<T: UserRepository> {
    repo: T,
}

// Bad: Depend on concrete implementation
struct UserService {
    repo: PostgresUserRepository,
}
```

### 3. Error Handling

- Define domain-specific errors
- Use `thiserror` for error types
- Convert between error types at boundaries

### 4. Testing

- Test domain logic in isolation
- Use mocks for external dependencies
- Write integration tests for critical paths

## Examples

### Domain Event

```rust
// domain/events/user_created.rs
pub struct UserCreated {
    pub user_id: UserId,
    pub timestamp: DateTime<Utc>,
}

impl DomainEvent for UserCreated {
    fn event_type(&self) -> &'static str {
        "user_created"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
```

### Command Handler

```rust
// application/commands/create_user.rs
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
}

pub struct CreateUserHandler<T: UserRepository, E: EventBus> {
    user_repo: Arc<T>,
    event_bus: Arc<E>,
}

#[async_trait]
impl<T, E> CommandHandler<CreateUserCommand> for CreateUserHandler<T, E>
where
    T: UserRepository,
    E: EventBus,
{
    async fn handle(&self, cmd: CreateUserCommand) -> Result<(), Error> {
        let user = User::new(cmd.username, cmd.email);
        self.user_repo.save(&user).await?;
        
        let event = UserCreated {
            user_id: user.id,
            timestamp: Utc::now(),
        };
        
        self.event_bus.publish(event).await?;
        Ok(())
    }
}
```

## Conclusion

The Hexagonal Architecture provides a clean separation of concerns, making the codebase more maintainable and testable. By following these patterns, we ensure that Anya Core remains flexible and adaptable to future changes.

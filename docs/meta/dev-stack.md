# Development Stack & Architecture Decisions

## Overview

The Kilter Board application is built with a focus on **performance**, **developer experience**, and **maintainability**. This document explains our technology choices and the reasoning behind them.

## Technology Stack

### Backend: Rust + Axum
**Choice**: Rust with Axum web framework
**Why**:
- **Performance**: Rust's zero-cost abstractions and memory safety without garbage collection
- **Type Safety**: Compile-time guarantees prevent entire classes of bugs
- **Ecosystem**: Excellent async support with Tokio, mature HTTP libraries
- **Axum**: Modern, ergonomic web framework with great performance
- **SQLx**: Compile-time verified SQL queries (no ORM overhead)

**Alternatives Considered**:
- Node.js/Express: Rejected due to runtime errors and performance concerns
- Go: Good choice, but Rust's type system provides better guarantees
- Python/FastAPI: Too slow for our performance requirements

### Mobile: React Native + TypeScript
**Choice**: React Native with TypeScript
**Why**:
- **Cross-platform**: Single codebase for iOS and Android
- **Performance**: Near-native performance with native modules
- **Developer Experience**: Hot reloading, excellent debugging tools
- **Ecosystem**: Mature library ecosystem, strong community
- **TypeScript**: Compile-time type checking prevents runtime errors

**Alternatives Considered**:
- Flutter: Rejected due to team's JavaScript/TypeScript expertise
- Native iOS/Android: Too much duplication of effort
- Ionic: Performance concerns for our use case

### Database: PostgreSQL
**Choice**: PostgreSQL 15+
**Why**:
- **ACID Compliance**: Critical for data integrity in climbing statistics
- **JSON Support**: Flexible schema for user preferences and metadata
- **Performance**: Excellent query optimization and indexing
- **Extensions**: PostGIS for potential location features, full-text search
- **Reliability**: Battle-tested in production environments

**Alternatives Considered**:
- MySQL: PostgreSQL has better JSON support and standards compliance
- MongoDB: ACID guarantees more important than document flexibility
- SQLite: Not suitable for multi-user concurrent access

### Caching: Redis
**Choice**: Redis 7
**Why**:
- **Performance**: In-memory storage for frequently accessed data
- **Data Structures**: Rich data types (sets, sorted sets) for leaderboards
- **Session Storage**: Fast session management
- **Pub/Sub**: Real-time features (future notifications)

### Container Runtime: OrbStack
**Choice**: OrbStack instead of Docker Desktop
**Why**:
- **Performance**: 2-5x faster than Docker Desktop on macOS
- **Resource Usage**: Much lower CPU and memory footprint
- **Native Integration**: Uses macOS Virtualization.framework
- **Compatibility**: Drop-in replacement for Docker commands
- **File Sync**: Faster bind mounts, no performance penalties

**Alternatives Considered**:
- Docker Desktop: Too resource-heavy, slower file sync
- Podman: Good alternative, but OrbStack has better macOS integration
- Native Development: Wanted containerization for consistency

### Task Runner: Just
**Choice**: Just instead of Make or npm scripts
**Why**:
- **Modern Syntax**: Clear, readable command definitions
- **Cross-platform**: Works consistently across macOS, Linux, Windows
- **Features**: Built-in help, parameter passing, conditional execution
- **Performance**: Fast startup, no shell overhead

### Python Package Management: uv
**Choice**: uv instead of pip/venv/poetry
**Why**:
- **Blazing Performance**: 10-100x faster than pip (written in Rust)
- **Zero Configuration**: Automatic virtual environment management
- **Reproducible Builds**: Lock files ensure consistent dependencies
- **Drop-in Replacement**: Familiar pip-like commands but way better
- **No Dependency Hell**: Proper dependency resolution that actually works
- **Modern Tooling**: Built-in project management and publishing support

### Testing Strategy: Dual Approach
**Choice**: Unit tests + Property-based testing
**Why**:
- **Unit Tests**: Specific examples, edge cases, integration points
- **Property Tests**: Universal properties across all inputs
- **Complementary**: Unit tests catch concrete bugs, property tests verify correctness
- **Frameworks**: Jest (mobile), QuickCheck (Rust), fast-check (TypeScript)

## Architecture Principles

### 1. **Separation of Concerns**
- Clear boundaries between transport, business logic, and data layers
- Domain-driven design with explicit models
- No business logic in controllers or UI components

### 2. **Type Safety First**
- Compile-time verification wherever possible
- Shared type definitions between frontend and backend
- No `any` types in TypeScript, strict Rust compiler settings

### 3. **Performance by Design**
- Async/await throughout the stack
- Connection pooling and caching strategies
- Minimal data transfer with efficient serialization

### 4. **Developer Experience**
- Hot reloading in development
- Fast feedback loops (tests, compilation)
- Clear error messages and debugging tools
- Consistent tooling across the team

### 5. **Correctness Properties**
- Property-based testing for core business logic
- Formal specifications for critical operations
- Round-trip testing for serialization/parsing

## Development Workflow

### Container-First Development
- All services run in containers for consistency
- Source code mounted for hot reloading
- Persistent volumes for data and build caches
- Easy onboarding with single command setup

### Testing Philosophy
- Test-driven development for critical paths
- Property-based testing for business logic
- Integration tests for API contracts
- End-to-end tests for user workflows

### Code Quality
- Automated formatting (rustfmt, prettier)
- Linting with strict rules
- Type checking in CI/CD
- Code review requirements

## Scalability Considerations

### Horizontal Scaling
- Stateless backend services
- Database connection pooling
- Redis for shared state
- Load balancer ready

### Performance Monitoring
- Structured logging with tracing
- Metrics collection points
- Database query optimization
- Caching strategies

### Security
- JWT-based authentication
- Input validation at API boundaries
- SQL injection prevention with SQLx
- CORS configuration
- Rate limiting (future)

## Future Considerations

### Potential Additions
- **GraphQL**: If mobile needs become more complex
- **WebSockets**: For real-time features (live competitions)
- **Microservices**: If team grows beyond 5-7 developers
- **CDN**: For static assets and global distribution
- **Monitoring**: Prometheus + Grafana for production metrics

### Migration Paths
- **Database**: PostgreSQL can scale to millions of users
- **Backend**: Rust services can be split into microservices
- **Mobile**: React Native can add native modules as needed
- **Caching**: Redis Cluster for high availability

## Trade-offs Made

### Rust Learning Curve
- **Trade-off**: Steeper learning curve vs. long-term maintainability
- **Mitigation**: Excellent documentation, strong type system helps

### React Native Limitations
- **Trade-off**: Some platform-specific features require native code
- **Mitigation**: Well-defined native module interface

### Container Overhead
- **Trade-off**: Slight resource overhead vs. development consistency
- **Mitigation**: OrbStack minimizes overhead, excellent caching

## Decision Log

| Decision | Date | Reasoning | Alternatives |
|----------|------|-----------|--------------|
| Rust Backend | 2024-12 | Performance + Safety | Node.js, Go, Python |
| React Native | 2024-12 | Cross-platform + DX | Flutter, Native |
| PostgreSQL | 2024-12 | ACID + JSON support | MySQL, MongoDB |
| OrbStack | 2024-12 | macOS performance | Docker Desktop, Podman |
| uv Package Manager | 2024-12 | Speed + Modern tooling | pip, poetry, conda |
| Property Testing | 2024-12 | Correctness guarantees | Unit tests only |

---

*This document is living and should be updated as architectural decisions evolve.*
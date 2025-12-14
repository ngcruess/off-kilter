# Meta Documentation

This directory contains documentation about the project's architecture, decisions, and development processes.

## Documents

### [Development Stack & Architecture Decisions](./dev-stack.md)
Comprehensive overview of our technology choices and the reasoning behind them:
- Technology stack rationale (Rust, React Native, PostgreSQL, etc.)
- Architecture principles and trade-offs
- Performance and scalability considerations
- Future migration paths and decision log

### [Getting Started Guide](./getting-started.md)
Step-by-step instructions for setting up the development environment from a fresh clone:
- Prerequisites and tool installation
- 5-minute quick start guide
- Development workflow and common commands
- Troubleshooting and debugging tips

## Quick Reference

### Essential Commands
```bash
just dev      # Start development environment
just health   # Check service status
just test     # Run all tests
just stop     # Stop all services
```

### Service URLs
- Backend API: http://localhost:3000
- PostgreSQL: localhost:5432
- Redis: localhost:6379

### Key Technologies
- **Backend**: Rust + Axum + SQLx + PostgreSQL
- **Mobile**: React Native + TypeScript + Jest
- **Containers**: OrbStack + Docker Compose
- **Testing**: Property-based testing + Unit tests

## Philosophy

This project prioritizes:
1. **Developer Experience**: Fast feedback loops, hot reloading, clear error messages
2. **Type Safety**: Compile-time guarantees prevent runtime errors
3. **Performance**: Rust backend, efficient React Native, optimized queries
4. **Correctness**: Property-based testing ensures business logic correctness
5. **Maintainability**: Clear architecture, comprehensive documentation

## Contributing

When making architectural decisions:
1. Document the decision in `dev-stack.md`
2. Update `getting-started.md` if setup changes
3. Consider impact on developer experience
4. Maintain backward compatibility where possible

---

*Keep this documentation up-to-date as the project evolves.*
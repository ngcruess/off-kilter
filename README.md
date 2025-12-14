# Kilter Board Application

A comprehensive cross-platform mobile application for interacting with Kilter Board climbing training systems.

## 🚀 Quick Start

**New to the project?** Start here: **[Getting Started Guide](./docs/meta/getting-started.md)**

```bash
# Install prerequisites: Rust, Node.js, OrbStack, Just
brew install node orbstack just

# Clone and setup
git clone <repo-url> && cd kilter-board-app
cd mobile && npm install && cd ..

# Start development environment (5 minutes)
just dev

# Verify everything works
just health
```

## 📚 Documentation

- **[Getting Started](./docs/meta/getting-started.md)** - Fresh clone setup guide
- **[Development Stack](./docs/meta/dev-stack.md)** - Architecture decisions and rationale
- **[Project Specs](./kiro/specs/kilter-board-app/)** - Requirements, design, and implementation plan

## 🏗️ Architecture

**Backend**: Rust + Axum + PostgreSQL + Redis  
**Mobile**: React Native + TypeScript  
**Containers**: OrbStack (fast Docker alternative)  
**Testing**: Property-based + Unit tests  

See [Development Stack](./docs/meta/dev-stack.md) for detailed rationale.

## 🛠️ Development Commands

```bash
just dev      # Start all services (containerized)
just health   # Check service status  
just test     # Run all tests
just logs     # View service logs
just stop     # Stop all services
```

## 📱 Features

- **Cross-platform mobile apps** (iOS/Android)
- **Boulder problem creation** with hold selection
- **Community voting system** (stars + difficulty)
- **Advanced search & filtering** by grade, tags, creator
- **Bluetooth board control** for hold illumination
- **Personal progress tracking** and statistics
- **Problem set curation** for training programs
- **Real-time data sync** across devices

## 🎯 Project Status

Currently implementing **Task 1**: Project structure and development environment ✅

Next up: Database setup and core data models

See [tasks.md](./.kiro/specs/kilter-board-app/tasks.md) for full implementation plan.

## 🤝 Contributing

1. Read the [Getting Started Guide](./docs/meta/getting-started.md)
2. Check the [Development Stack](./docs/meta/dev-stack.md) for architecture
3. Follow the implementation plan in [tasks.md](./.kiro/specs/kilter-board-app/tasks.md)

## 📄 License

MIT License
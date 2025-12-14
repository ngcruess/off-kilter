# Getting Started - Fresh Clone Setup

This guide will get you up and running with the Kilter Board application from a fresh clone of the repository.

## Prerequisites

### Required Tools
- **Rust** (1.83+): [Install via rustup](https://rustup.rs/)
- **Node.js** (18+): [Install via official installer](https://nodejs.org/) or `brew install node`
- **OrbStack**: [Install via Homebrew](https://orbstack.dev/) - `brew install orbstack`
- **Just**: Task runner - `brew install just`
- **uv**: Python package manager - `brew install uv`

### Platform-Specific Requirements

#### macOS
```bash
# Install Xcode Command Line Tools (for React Native)
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install all tools at once
brew install node orbstack just uv
```

#### Linux
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Just
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin
```

## Quick Start (5 minutes)

### 1. Clone and Setup
```bash
# Clone the repository
git clone <repository-url>
cd kilter-board-app

# Install mobile dependencies
cd mobile && npm install && cd ..

# Verify Rust installation
cargo --version
```

### 2. Start Development Environment
```bash
# Start all services (databases + backend in containers)
just dev

# This will:
# - Pull and start PostgreSQL and Redis containers
# - Build and start the Rust backend with hot reloading
# - Display service URLs when ready
```

### 3. Verify Everything Works
```bash
# Check service health
just health

# Should show:
# ✅ Backend: OK
# All containers running and healthy
```

### 4. Test Mobile App
```bash
# In a new terminal
cd mobile

# Run tests
npm test

# Type check
npm run typecheck

# Start Metro bundler (for development)
npm start
```

## Development Workflow

### Daily Development
```bash
# Start your day
just dev                    # Start all services
just health                 # Verify everything is running

# During development
just logs backend-dev       # View backend logs
just logs postgres         # View database logs
just test                  # Run all tests

# End of day
just stop                  # Stop all services
```

### Common Commands
```bash
# Backend development
cd backend
cargo test                 # Run Rust tests
cargo check               # Quick compile check
cargo fmt                 # Format code

# Mobile development  
cd mobile
npm test                  # Run Jest tests
npm run typecheck        # TypeScript checking
npm run lint             # ESLint checking

# Database access
docker exec -it off-kilter-postgres-1 psql -U postgres -d kilter_board
```

## Project Structure

```
kilter-board-app/
├── backend/              # Rust API server
│   ├── src/             # Source code
│   ├── tests/           # Integration tests
│   ├── Cargo.toml       # Dependencies
│   └── Dockerfile.dev   # Development container
├── mobile/              # React Native app
│   ├── src/             # TypeScript source
│   ├── __tests__/       # Jest tests
│   ├── package.json     # Node dependencies
│   └── tsconfig.json    # TypeScript config
├── docs/                # Documentation
│   └── meta/           # Architecture docs
├── docker-compose.dev.yml  # Full development stack
├── docker-compose.services.yml  # Just databases
├── justfile            # Task runner commands
└── README.md           # Project overview
```

## Service URLs

When running `just dev`, these services will be available:

- **Backend API**: http://localhost:3000
- **PostgreSQL**: localhost:5432 (user: postgres, password: postgres, db: kilter_board)
- **Redis**: localhost:6379

## Troubleshooting

### OrbStack Issues
```bash
# Check OrbStack status
orb status

# Start OrbStack if stopped
orb start

# Reset if having issues
orb reset
```

### Container Issues
```bash
# View all container logs
just logs

# Rebuild containers from scratch
just stop
docker system prune -f
just dev
```

### Backend Issues
```bash
# Check Rust installation
rustc --version
cargo --version

# Clean and rebuild
cd backend
cargo clean
cargo build
```

### Mobile Issues
```bash
# Clear npm cache
cd mobile
npm cache clean --force
rm -rf node_modules
npm install

# Reset Metro bundler
npx react-native start --reset-cache
```

### Database Connection Issues
```bash
# Check if PostgreSQL is running
docker ps | grep postgres

# Connect to database manually
docker exec -it off-kilter-postgres-1 psql -U postgres -d kilter_board

# View database logs
just logs postgres
```

## Development Tips

### Hot Reloading
- **Backend**: Automatically reloads when you save Rust files
- **Mobile**: Metro bundler provides hot reloading for React Native
- **Database**: Schema changes require manual migration

### Testing
```bash
# Run specific test suites
cd backend && cargo test integration_test
cd mobile && npm test -- App.test.tsx

# Run tests in watch mode
cd mobile && npm test -- --watch
```

### Debugging
- **Backend**: Use `println!` or `tracing::info!` for logging
- **Mobile**: Use React Native Debugger or Flipper
- **Database**: Use `just logs postgres` to see query logs

### Performance
- **First build**: Takes 5-10 minutes (downloads dependencies)
- **Subsequent builds**: 30 seconds - 2 minutes (cached)
- **Hot reloads**: 1-3 seconds

## IDE Setup

### VS Code (Recommended)
Install these extensions:
- **rust-analyzer**: Rust language support
- **React Native Tools**: React Native debugging
- **TypeScript Importer**: Auto-import for TypeScript
- **Docker**: Container management
- **Thunder Client**: API testing

### Settings
```json
{
  "rust-analyzer.cargo.features": "all",
  "typescript.preferences.importModuleSpecifier": "relative",
  "editor.formatOnSave": true
}
```

## Next Steps

Once you have everything running:

1. **Explore the API**: Visit http://localhost:3000 
2. **Run the mobile app**: Follow React Native setup for iOS/Android
3. **Read the specs**: Check `.kiro/specs/kilter-board-app/` for requirements and design
4. **Start implementing**: Follow the task list in `tasks.md`

## Getting Help

- **Architecture Questions**: See `docs/meta/dev-stack.md`
- **API Documentation**: Will be available at http://localhost:3000/docs (future)
- **Database Schema**: Check `backend/migrations/` (future)
- **Mobile Components**: See `mobile/src/components/` (future)

---

**Estimated setup time**: 5-15 minutes (depending on download speeds)
**Estimated learning time**: 1-2 hours to understand the full stack
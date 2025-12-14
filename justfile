# Kilter Board Development Commands
# Install just: brew install just

# Default recipe - show available commands
default:
    @just --list

# Development environment setup
setup:
    @echo "🚀 Setting up Kilter Board development environment..."
    @echo "Choose your container runtime:"
    @echo "1. OrbStack (recommended for macOS): brew install orbstack"
    @echo "2. Podman: brew install podman podman-compose"
    @echo "3. Colima: brew install colima"
    @echo "4. Docker Desktop (if you must)"

# Start development environment (containerized backend + services)
dev:
    @echo "🏗️  Starting development environment..."
    docker-compose -f docker-compose.dev.yml up -d
    @echo "✅ Services started!"
    @echo "📊 Backend: http://localhost:3000 (containerized with hot reload)"
    @echo "🗄️  PostgreSQL: localhost:5432"
    @echo "🔴 Redis: localhost:6379"

# Start only services (databases) - run backend natively
dev-services:
    @echo "🏗️  Starting development services..."
    docker-compose -f docker-compose.services.yml up -d
    @echo "✅ Services started!"
    @echo "🗄️  PostgreSQL: localhost:5432"
    @echo "🔴 Redis: localhost:6379"
    @echo ""
    @echo "To start the backend natively:"
    @echo "  cd backend && cargo run"

# Stop development environment
stop:
    @echo "🛑 Stopping development environment..."
    docker-compose -f docker-compose.dev.yml down

# Stop only services
stop-services:
    @echo "🛑 Stopping development services..."
    docker-compose -f docker-compose.services.yml down

# View logs
logs service="":
    #!/usr/bin/env bash
    if [ -z "{{service}}" ]; then
        docker-compose -f docker-compose.dev.yml logs -f
    else
        docker-compose -f docker-compose.dev.yml logs -f {{service}}
    fi

# Clean up everything (careful!)
clean:
    @echo "🧹 Cleaning up containers and volumes..."
    docker-compose -f docker-compose.dev.yml down -v
    docker system prune -f

# Backend commands
backend-test:
    @echo "🧪 Running backend tests..."
    cd backend && cargo test

backend-check:
    @echo "🔍 Checking backend code..."
    cd backend && cargo check

backend-fmt:
    @echo "🎨 Formatting backend code..."
    cd backend && cargo fmt

# Mobile commands
mobile-install:
    @echo "📱 Installing mobile dependencies..."
    cd mobile && npm install

mobile-test:
    @echo "🧪 Running mobile tests..."
    cd mobile && npm test

mobile-typecheck:
    @echo "🔍 Type checking mobile code..."
    cd mobile && npm run typecheck

mobile-lint:
    @echo "🎨 Linting mobile code..."
    cd mobile && npm run lint

# Run all tests
test: backend-test mobile-test
    @echo "✅ All tests completed!"

# Format all code
fmt: backend-fmt
    @echo "🎨 Formatting mobile code..."
    cd mobile && npm run lint --fix
    @echo "✅ All code formatted!"

# Health check
health:
    @echo "🏥 Checking service health..."
    @curl -s http://localhost:3000 > /dev/null && echo "✅ Backend: OK" || echo "❌ Backend: DOWN"
    @docker-compose -f docker-compose.dev.yml ps
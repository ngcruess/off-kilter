# Implementation Plan

- [ ] 1. Set up project structure and development environment
  - Create Rust backend project with Cargo workspace
  - Set up React Native project with TypeScript
  - Configure development tools (linting, formatting, testing)
  - Set up Docker containers for local development
  - _Requirements: All requirements depend on proper project setup_

- [ ] 2. Set up database and authentication infrastructure
  - Configure PostgreSQL database with initial schema
  - Set up Supabase project and authentication
  - Create database migration system
  - Configure connection pooling and environment variables
  - _Requirements: 1.1, 1.2, 7.2_

- [ ] 3. Implement core data models and validation
- [ ] 3.1 Create Rust data structures for core entities
  - Implement User, BoulderProblem, Vote, and related structs
  - Add serde serialization/deserialization
  - Implement validation logic for grades, tags, and hold configurations
  - _Requirements: 2.5, 2.6, 2.7, 10.3, 10.4_

- [ ]* 3.2 Write property test for JSON serialization round-trip
  - **Property 4: JSON Serialization Round-Trip**
  - **Validates: Requirements 2.8, 10.3, 10.4, 10.5**

- [ ]* 3.3 Write property test for tag validation
  - **Property 3: Tag Validation Consistency**
  - **Validates: Requirements 2.6, 2.7**

- [ ] 3.4 Implement database schema and SQLx queries
  - Create all database tables with proper relationships
  - Implement CRUD operations using SQLx
  - Add database indexes for search performance
  - _Requirements: 10.1, 10.6_

- [ ]* 3.5 Write unit tests for data model validation
  - Test grade validation (V-scale format)
  - Test tag regex pattern matching
  - Test hold configuration validation
  - _Requirements: 2.5, 2.6, 2.7_

- [ ] 4. Build authentication and user management system
- [ ] 4.1 Implement Supabase JWT validation in Rust
  - Create JWT middleware for Axum
  - Implement user extraction from tokens
  - Add authentication error handling
  - _Requirements: 1.2, 7.1_

- [ ] 4.2 Create user management API endpoints
  - Implement user profile CRUD operations
  - Add climbing statistics tracking
  - Create user history and achievement endpoints
  - _Requirements: 1.3, 1.5, 1.6, 9.2, 9.3, 9.5_

- [ ]* 4.3 Write property test for user account lifecycle
  - **Property 1: User Account Lifecycle Consistency**
  - **Validates: Requirements 1.1, 1.2, 1.5**

- [ ]* 4.4 Write unit tests for authentication middleware
  - Test JWT validation with valid/invalid tokens
  - Test user extraction and error cases
  - Test authentication flow integration
  - _Requirements: 1.2, 7.1_

- [ ] 5. Implement boulder problem management system
- [ ] 5.1 Create problem CRUD API endpoints
  - Implement problem creation with validation
  - Add problem editing while preserving votes/attempts
  - Create problem publishing and visibility controls
  - _Requirements: 2.1, 2.3, 2.4, 2.5_

- [ ] 5.2 Implement tag management system
  - Create predefined tag system with validation
  - Add tag assignment and removal for problems
  - Implement tag-based filtering logic
  - _Requirements: 2.6, 2.7_

- [ ]* 5.3 Write property test for problem creation validation
  - **Property 2: Problem Creation Validation**
  - **Validates: Requirements 2.5**

- [ ]* 5.4 Write unit tests for problem management
  - Test problem creation with required fields
  - Test problem editing preserves votes/attempts
  - Test tag assignment and validation
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 2.6, 2.7_

- [ ] 6. Build voting and rating system
- [ ] 6.1 Implement dual-dimension voting API
  - Create vote submission endpoints (stars + difficulty)
  - Implement vote update logic (no duplicates)
  - Add vote retrieval and user vote history
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ] 6.2 Create rating aggregation system
  - Implement average star rating calculation
  - Add consensus difficulty grade computation
  - Create rating display endpoints with distributions
  - _Requirements: 4.5, 4.6, 4.7_

- [ ]* 6.3 Write property test for dual-dimension vote recording
  - **Property 8: Dual-Dimension Vote Recording**
  - **Validates: Requirements 4.2**

- [ ]* 6.4 Write property test for vote uniqueness
  - **Property 9: Vote Uniqueness Constraint**
  - **Validates: Requirements 4.4**

- [ ]* 6.5 Write property test for rating aggregation
  - **Property 10: Rating Aggregation Accuracy**
  - **Validates: Requirements 4.6**

- [ ]* 6.6 Write unit tests for voting system
  - Test vote submission and update flows
  - Test rating calculation accuracy
  - Test vote permission validation
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ] 7. Implement search and discovery system
- [ ] 7.1 Create search API with multiple filters
  - Implement grade range filtering
  - Add creator and keyword search
  - Create tag-based filtering with AND logic
  - Add sorting by ascents and star ratings
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7_

- [ ]* 7.2 Write property test for search filter consistency
  - **Property 5: Search Filter Consistency**
  - **Validates: Requirements 3.1**

- [ ]* 7.3 Write property test for tag filter AND logic
  - **Property 6: Tag Filter AND Logic**
  - **Validates: Requirements 3.4**

- [ ]* 7.4 Write property test for search result sorting
  - **Property 7: Search Result Sorting**
  - **Validates: Requirements 3.6, 3.7**

- [ ]* 7.5 Write unit tests for search functionality
  - Test individual filter types
  - Test combined filter scenarios
  - Test sorting and pagination
  - Test empty result handling
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

- [ ] 8. Build problem set management system
- [ ] 8.1 Implement problem set CRUD operations
  - Create problem set creation and editing
  - Add problem addition/removal from sets
  - Implement set publishing and discovery
  - Create difficulty range calculation for sets
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ]* 8.2 Write unit tests for problem set management
  - Test set creation and modification
  - Test problem addition/removal
  - Test difficulty range calculation
  - Test set discovery and display
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 9. Checkpoint - Backend API completion
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. Set up React Native frontend foundation
- [ ] 10.1 Create React Native project structure
  - Set up navigation with React Navigation
  - Configure state management with React Query
  - Add TypeScript configuration and types
  - Set up development tools and debugging
  - _Requirements: 6.1, 6.4_

- [ ] 10.2 Implement Supabase authentication integration
  - Add Supabase client configuration
  - Create authentication screens (login/register)
  - Implement Google and Apple sign-in
  - Add authentication state management
  - _Requirements: 1.1, 1.2, 1.4_

- [ ]* 10.3 Write unit tests for authentication screens
  - Test login/register form validation
  - Test social sign-in integration
  - Test authentication state transitions
  - _Requirements: 1.1, 1.2, 1.4_

- [ ] 11. Build core mobile app screens
- [ ] 11.1 Create user profile and statistics screens
  - Implement profile viewing and editing
  - Add climbing statistics display
  - Create user history and achievements view
  - Add account deletion functionality
  - _Requirements: 1.3, 1.7, 9.3, 9.4, 9.5_

- [ ] 11.2 Implement problem browsing and search
  - Create problem list with filtering options
  - Add search interface with multiple criteria
  - Implement sorting and pagination
  - Add problem detail view with ratings
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

- [ ] 11.3 Create problem creation and editing screens
  - Build hold selection interface
  - Add problem metadata entry forms
  - Implement tag selection from predefined list
  - Add problem publishing workflow
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 2.6, 2.7_

- [ ] 11.4 Implement voting and rating interface
  - Create dual-dimension voting UI (stars + difficulty)
  - Add vote submission and update functionality
  - Display aggregate ratings and distributions
  - Show user's previous votes
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ]* 11.5 Write unit tests for core screens
  - Test screen rendering and navigation
  - Test form validation and submission
  - Test data loading and error states
  - Test user interaction flows
  - _Requirements: Various UI-related requirements_

- [ ] 12. Implement Bluetooth integration
- [ ] 12.1 Set up Bluetooth communication module
  - Add react-native-ble-plx dependency
  - Create Bluetooth service for board communication
  - Implement board discovery and connection
  - Add connection state management
  - _Requirements: 8.1, 8.2, 8.4, 8.5_

- [ ] 12.2 Create hold illumination system
  - Implement hold configuration parsing
  - Add board control commands for lighting
  - Create real-time hold pattern updates
  - Add board model and firmware verification
  - _Requirements: 8.1, 8.2, 8.3_

- [ ]* 12.3 Write property test for Bluetooth hold illumination
  - **Property 12: Bluetooth Hold Illumination**
  - **Validates: Requirements 8.1**

- [ ]* 12.4 Write unit tests for Bluetooth integration
  - Test board discovery and connection
  - Test hold illumination commands
  - Test error handling and reconnection
  - Test firmware compatibility checks
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [ ] 13. Implement attempt tracking system
- [ ] 13.1 Create attempt recording functionality
  - Add attempt logging with timestamps
  - Implement ascent tracking and statistics updates
  - Create personal achievement calculation
  - Add progress tracking and milestone detection
  - _Requirements: 9.1, 9.2, 9.5_

- [ ]* 13.2 Write property test for attempt tracking
  - **Property 13: Attempt Tracking Completeness**
  - **Validates: Requirements 9.1**

- [ ]* 13.3 Write property test for statistics updates
  - **Property 14: Statistics Update Consistency**
  - **Validates: Requirements 9.2**

- [ ]* 13.4 Write unit tests for attempt tracking
  - Test attempt recording accuracy
  - Test statistics calculation
  - Test achievement detection
  - Test progress tracking
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [ ] 14. Add cross-platform data synchronization
- [ ] 14.1 Implement data sync and offline support
  - Add offline data caching
  - Create sync conflict resolution
  - Implement background data refresh
  - Add cross-device consistency checks
  - _Requirements: 6.2, 6.5_

- [ ]* 14.2 Write property test for cross-platform synchronization
  - **Property 11: Cross-Platform Data Synchronization**
  - **Validates: Requirements 6.2**

- [ ]* 14.3 Write unit tests for data synchronization
  - Test offline mode functionality
  - Test sync conflict resolution
  - Test cross-device data consistency
  - Test background refresh behavior
  - _Requirements: 6.2, 6.5_

- [ ] 15. Implement data persistence and recovery
- [ ] 15.1 Add comprehensive error handling and recovery
  - Implement data corruption detection
  - Add automated recovery procedures
  - Create administrator alert system
  - Add backup and restore functionality
  - _Requirements: 7.5, 10.6_

- [ ]* 15.2 Write property test for data persistence
  - **Property 15: Data Persistence Immediacy**
  - **Validates: Requirements 10.1**

- [ ]* 15.3 Write unit tests for error handling
  - Test corruption detection and recovery
  - Test backup and restore procedures
  - Test administrator alert system
  - Test graceful error handling
  - _Requirements: 7.5, 10.6_

- [ ] 16. Final integration and deployment preparation
- [ ] 16.1 Set up deployment infrastructure
  - Create Docker containers for production
  - Configure Kubernetes deployment manifests
  - Set up CI/CD pipeline with automated testing
  - Add monitoring and logging infrastructure
  - _Requirements: 7.4, 7.5_

- [ ] 16.2 Perform end-to-end integration testing
  - Test complete user workflows
  - Verify cross-platform functionality
  - Test Bluetooth integration with real hardware
  - Validate performance under load
  - _Requirements: 6.1, 6.4, 7.3, 7.4_

- [ ]* 16.3 Write integration tests for critical workflows
  - Test user registration to problem creation flow
  - Test problem search to Bluetooth illumination flow
  - Test voting and rating aggregation flow
  - Test cross-platform data sync flow
  - _Requirements: Various end-to-end workflows_

- [ ] 17. Final checkpoint - Complete system verification
  - Ensure all tests pass, ask the user if questions arise.
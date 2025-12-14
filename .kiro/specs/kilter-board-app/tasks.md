# Implementation Plan - Feature-Driven Vertical Slices

## Approach

This implementation plan follows a **vertical slice** approach where each feature is built end-to-end (models → database → API → tests) before moving to the next feature. This provides faster feedback loops, better risk management, and more satisfying progress.

---

- [x] 1. Set up project structure and development environment
  - Create Rust backend project with Cargo workspace
  - Set up React Native project with TypeScript
  - Configure development tools (linting, formatting, testing)
  - Set up Docker containers for local development
  - _Requirements: All requirements depend on proper project setup_

## Core Infrastructure

- [ ] 2. Database and authentication foundation
- [x] 2.1 Set up database infrastructure
  - Configure PostgreSQL database with connection pooling
  - Create database migration system with SQLx
  - Set up environment variables and configuration
  - _Requirements: 7.2, 10.1_

- [-] 2.2 Implement basic authentication middleware
  - Create JWT validation middleware for Axum
  - Implement user extraction from tokens
  - Add authentication error handling
  - _Requirements: 1.2, 7.1_

- [ ]* 2.3 Write unit tests for authentication middleware
  - Test JWT validation with valid/invalid tokens
  - Test user extraction and error cases
  - _Requirements: 1.2, 7.1_

## Feature 1: User Account Management

- [ ] 3. User account lifecycle (complete vertical slice)
- [ ] 3.1 Create User data model and database schema
  - Implement User struct with serde serialization
  - Create users table with proper indexes
  - Add user statistics and profile fields
  - _Requirements: 1.1, 1.5, 9.2, 9.3_

- [ ] 3.2 Implement user management API endpoints
  - POST /users (registration)
  - GET /users/me (profile)
  - PUT /users/me (update profile)
  - DELETE /users/me (account deletion)
  - _Requirements: 1.1, 1.3, 1.7_

- [ ]* 3.3 Write property test for user account lifecycle
  - **Property 1: User Account Lifecycle Consistency**
  - **Validates: Requirements 1.1, 1.2, 1.5**

- [ ]* 3.4 Write unit tests for user management
  - Test user creation and validation
  - Test profile updates and statistics tracking
  - Test account deletion preserves anonymized data
  - _Requirements: 1.1, 1.3, 1.5, 1.7_

- [ ] 3.5 Test user management end-to-end
  - Manual API testing with curl/Postman
  - Verify database state after operations
  - Test error cases and edge conditions

## Feature 2: Boulder Problem Creation

- [ ] 4. Boulder problem creation (complete vertical slice)
- [ ] 4.1 Create BoulderProblem data model and database schema
  - Implement BoulderProblem struct with hold configurations
  - Create boulder_problems table with relationships
  - Add validation for grades, tags, and hold patterns
  - _Requirements: 2.1, 2.5, 2.6, 2.7_

- [ ] 4.2 Implement problem creation API endpoints
  - POST /problems (create problem)
  - GET /problems/:id (get problem details)
  - PUT /problems/:id (edit problem, preserve votes)
  - DELETE /problems/:id (soft delete)
  - _Requirements: 2.1, 2.3, 2.4_

- [ ]* 4.3 Write property test for problem creation validation
  - **Property 2: Problem Creation Validation**
  - **Validates: Requirements 2.5**

- [ ]* 4.4 Write property test for tag validation
  - **Property 3: Tag Validation Consistency**
  - **Validates: Requirements 2.6, 2.7**

- [ ]* 4.5 Write property test for JSON serialization
  - **Property 4: JSON Serialization Round-Trip**
  - **Validates: Requirements 2.8, 10.3, 10.4, 10.5**

- [ ]* 4.6 Write unit tests for problem management
  - Test problem creation with required fields
  - Test problem editing preserves votes/attempts
  - Test tag assignment and validation
  - Test hold configuration validation
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 2.6, 2.7_

- [ ] 4.7 Test problem creation end-to-end
  - Create problems via API with various configurations
  - Verify database storage and retrieval
  - Test validation edge cases

## Feature 3: Voting and Rating System

- [ ] 5. Dual-dimension voting (complete vertical slice)
- [ ] 5.1 Create Vote data model and database schema
  - Implement Vote struct with star rating and difficulty
  - Create votes table with unique constraints
  - Add rating aggregation views/functions
  - _Requirements: 4.1, 4.2, 4.4_

- [ ] 5.2 Implement voting API endpoints
  - POST /problems/:id/votes (submit/update vote)
  - GET /problems/:id/votes (get vote distribution)
  - GET /users/me/votes (user's voting history)
  - GET /problems/:id/ratings (aggregate ratings)
  - _Requirements: 4.1, 4.2, 4.3, 4.5, 4.6, 4.7_

- [ ]* 5.3 Write property test for dual-dimension vote recording
  - **Property 8: Dual-Dimension Vote Recording**
  - **Validates: Requirements 4.2**

- [ ]* 5.4 Write property test for vote uniqueness
  - **Property 9: Vote Uniqueness Constraint**
  - **Validates: Requirements 4.4**

- [ ]* 5.5 Write property test for rating aggregation
  - **Property 10: Rating Aggregation Accuracy**
  - **Validates: Requirements 4.6**

- [ ]* 5.6 Write unit tests for voting system
  - Test vote submission and update flows
  - Test rating calculation accuracy
  - Test vote permission validation
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ] 5.7 Test voting system end-to-end
  - Submit votes and verify aggregation
  - Test vote updates and uniqueness constraints
  - Verify rating calculations

## Feature 4: Search and Discovery

- [ ] 6. Problem search and filtering (complete vertical slice)
- [ ] 6.1 Implement search API with multiple filters
  - GET /problems/search with query parameters
  - Grade range filtering (min_grade, max_grade)
  - Creator and keyword search
  - Tag-based filtering with AND logic
  - Sorting by ascents, stars, created date
  - Pagination support
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

- [ ]* 6.2 Write property test for search filter consistency
  - **Property 5: Search Filter Consistency**
  - **Validates: Requirements 3.1**

- [ ]* 6.3 Write property test for tag filter AND logic
  - **Property 6: Tag Filter AND Logic**
  - **Validates: Requirements 3.4**

- [ ]* 6.4 Write property test for search result sorting
  - **Property 7: Search Result Sorting**
  - **Validates: Requirements 3.6, 3.7**

- [ ]* 6.5 Write unit tests for search functionality
  - Test individual filter types
  - Test combined filter scenarios
  - Test sorting and pagination
  - Test empty result handling
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

- [ ] 6.6 Test search system end-to-end
  - Create test data with various attributes
  - Test all filter combinations
  - Verify sorting and pagination

## Feature 5: Problem Sets

- [ ] 7. Problem set management (complete vertical slice)
- [ ] 7.1 Create ProblemSet data model and database schema
  - Implement ProblemSet struct with problem references
  - Create problem_sets table with many-to-many relationships
  - Add difficulty range calculation
  - _Requirements: 5.1, 5.2, 5.5_

- [ ] 7.2 Implement problem set API endpoints
  - POST /problem-sets (create set)
  - GET /problem-sets/:id (get set details)
  - PUT /problem-sets/:id (update set)
  - POST /problem-sets/:id/problems (add problem to set)
  - DELETE /problem-sets/:id/problems/:problem_id (remove from set)
  - _Requirements: 5.1, 5.2, 5.3, 5.4_

- [ ]* 7.3 Write unit tests for problem set management
  - Test set creation and modification
  - Test problem addition/removal
  - Test difficulty range calculation
  - Test set discovery and display
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 7.4 Test problem sets end-to-end
  - Create sets and add/remove problems
  - Verify difficulty calculations
  - Test set discovery

## Feature 6: Attempt Tracking

- [ ] 8. User attempt tracking (complete vertical slice)
- [ ] 8.1 Create UserAttempt data model and database schema
  - Implement UserAttempt struct with timestamps
  - Create user_attempts table with proper indexes
  - Add statistics calculation triggers/functions
  - _Requirements: 9.1, 9.2_

- [ ] 8.2 Implement attempt tracking API endpoints
  - POST /problems/:id/attempts (record attempt)
  - GET /users/me/attempts (user's attempt history)
  - GET /users/me/statistics (calculated statistics)
  - GET /users/me/achievements (milestones and records)
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [ ]* 8.3 Write property test for attempt tracking
  - **Property 13: Attempt Tracking Completeness**
  - **Validates: Requirements 9.1**

- [ ]* 8.4 Write property test for statistics updates
  - **Property 14: Statistics Update Consistency**
  - **Validates: Requirements 9.2**

- [ ]* 8.5 Write unit tests for attempt tracking
  - Test attempt recording accuracy
  - Test statistics calculation
  - Test achievement detection
  - Test progress tracking
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [ ] 8.6 Test attempt tracking end-to-end
  - Record attempts and verify statistics
  - Test achievement calculations
  - Verify progress tracking

## Backend API Completion

- [ ] 9. Backend integration and testing
- [ ] 9.1 Integration testing across features
  - Test user creates problem, votes, attempts workflow
  - Test search finds problems with votes and attempts
  - Test problem sets with user statistics
  - _Requirements: Cross-feature integration_

- [ ]* 9.2 Write property test for data persistence
  - **Property 15: Data Persistence Immediacy**
  - **Validates: Requirements 10.1**

- [ ] 9.3 Performance testing and optimization
  - Database query optimization
  - API response time testing
  - Connection pooling verification
  - _Requirements: 7.3, 7.4_

- [ ] 9.4 Checkpoint - Backend API completion
  - Ensure all tests pass, ask the user if questions arise.

## Mobile Application Features

- [ ] 10. Mobile app foundation
- [ ] 10.1 Set up React Native project structure
  - Configure navigation with React Navigation
  - Set up state management with React Query
  - Add TypeScript types from backend
  - Configure development tools and debugging
  - _Requirements: 6.1, 6.4_

- [ ] 10.2 Implement authentication screens
  - Login/register screens with form validation
  - Supabase authentication integration
  - JWT token management
  - Authentication state management
  - _Requirements: 1.1, 1.2, 1.4_

- [ ]* 10.3 Write unit tests for authentication screens
  - Test form validation and submission
  - Test authentication state transitions
  - _Requirements: 1.1, 1.2, 1.4_

## Mobile Feature 1: User Profile

- [ ] 11. User profile and statistics (mobile vertical slice)
- [ ] 11.1 Create user profile screens
  - Profile viewing and editing screen
  - Climbing statistics display
  - User history and achievements view
  - Account settings and deletion
  - _Requirements: 1.3, 1.7, 9.3, 9.4, 9.5_

- [ ]* 11.2 Write unit tests for profile screens
  - Test screen rendering and navigation
  - Test form validation and API integration
  - _Requirements: 1.3, 1.7_

## Mobile Feature 2: Problem Management

- [ ] 12. Problem creation and browsing (mobile vertical slice)
- [ ] 12.1 Create problem browsing screens
  - Problem list with filtering options
  - Search interface with multiple criteria
  - Problem detail view with ratings
  - Sorting and pagination
  - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

- [ ] 12.2 Create problem creation screens
  - Hold selection interface
  - Problem metadata entry forms
  - Tag selection from predefined list
  - Problem publishing workflow
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 2.6, 2.7_

- [ ]* 12.3 Write unit tests for problem screens
  - Test screen rendering and navigation
  - Test form validation and submission
  - Test search and filtering functionality
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 3.1, 3.2_

## Mobile Feature 3: Voting Interface

- [ ] 13. Voting and rating interface (mobile vertical slice)
- [ ] 13.1 Create voting screens
  - Dual-dimension voting UI (stars + difficulty)
  - Vote submission and update functionality
  - Aggregate ratings display with distributions
  - User's voting history
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ]* 13.2 Write unit tests for voting interface
  - Test voting UI components
  - Test vote submission and updates
  - Test rating display accuracy
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

## Hardware Integration

- [ ] 14. Bluetooth board integration (complete vertical slice)
- [ ] 14.1 Set up Bluetooth communication
  - Add react-native-ble-plx dependency
  - Create Bluetooth service for board communication
  - Implement board discovery and connection
  - Add connection state management
  - _Requirements: 8.1, 8.2, 8.4, 8.5_

- [ ] 14.2 Implement hold illumination system
  - Parse hold configurations from problems
  - Send board control commands for lighting
  - Real-time hold pattern updates
  - Board model and firmware verification
  - _Requirements: 8.1, 8.2, 8.3_

- [ ]* 14.3 Write property test for Bluetooth hold illumination
  - **Property 12: Bluetooth Hold Illumination**
  - **Validates: Requirements 8.1**

- [ ]* 14.4 Write unit tests for Bluetooth integration
  - Test board discovery and connection
  - Test hold illumination commands
  - Test error handling and reconnection
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [ ] 14.5 Test Bluetooth integration end-to-end
  - Connect to real hardware
  - Test hold illumination patterns
  - Verify board compatibility

## Advanced Features

- [ ] 15. Cross-platform data synchronization
- [ ] 15.1 Implement offline support and sync
  - Add offline data caching
  - Create sync conflict resolution
  - Implement background data refresh
  - Add cross-device consistency checks
  - _Requirements: 6.2, 6.5_

- [ ]* 15.2 Write property test for cross-platform synchronization
  - **Property 11: Cross-Platform Data Synchronization**
  - **Validates: Requirements 6.2**

- [ ]* 15.3 Write unit tests for data synchronization
  - Test offline mode functionality
  - Test sync conflict resolution
  - Test cross-device data consistency
  - _Requirements: 6.2, 6.5_

## Production Readiness

- [ ] 16. Error handling and monitoring
- [ ] 16.1 Comprehensive error handling
  - Implement data corruption detection
  - Add automated recovery procedures
  - Create administrator alert system
  - Add backup and restore functionality
  - _Requirements: 7.5, 10.6_

- [ ]* 16.2 Write unit tests for error handling
  - Test corruption detection and recovery
  - Test backup and restore procedures
  - Test administrator alert system
  - _Requirements: 7.5, 10.6_

- [ ] 17. Deployment and infrastructure
- [ ] 17.1 Set up production deployment
  - Create production Docker containers
  - Configure deployment manifests
  - Set up CI/CD pipeline with automated testing
  - Add monitoring and logging infrastructure
  - _Requirements: 7.4, 7.5_

- [ ] 17.2 End-to-end system testing
  - Test complete user workflows
  - Verify cross-platform functionality
  - Test Bluetooth integration with real hardware
  - Validate performance under load
  - _Requirements: 6.1, 6.4, 7.3, 7.4_

- [ ]* 17.3 Write integration tests for critical workflows
  - Test user registration to problem creation flow
  - Test problem search to Bluetooth illumination flow
  - Test voting and rating aggregation flow
  - Test cross-platform data sync flow
  - _Requirements: Various end-to-end workflows_

- [ ] 18. Final system verification
  - Ensure all tests pass, ask the user if questions arise.

---

## Benefits of This Approach

- **Faster Feedback**: Each feature is testable end-to-end immediately
- **Risk Reduction**: Integration issues discovered early
- **Better Testing**: Property-based tests can validate complete workflows
- **Satisfying Progress**: Working features vs. incomplete layers
- **Easier Debugging**: Complete context for each feature
- **Flexible Prioritization**: Features can be reordered based on user needs
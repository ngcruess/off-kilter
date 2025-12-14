#!/usr/bin/env python3
"""
Test script for user management API endpoints
"""

import requests
import json
import sys
from datetime import datetime, timedelta
import jwt

# Configuration
BASE_URL = "http://localhost:3000"
JWT_SECRET = "your-super-secret-jwt-key-change-this-in-production-please"  # Should match backend/.env

def create_test_jwt(user_id: str, email: str, username: str) -> str:
    """Create a test JWT token for authentication"""
    payload = {
        "sub": user_id,
        "email": email,
        "username": username,
        "exp": datetime.utcnow() + timedelta(hours=1),
        "iat": datetime.utcnow()
    }
    return jwt.encode(payload, JWT_SECRET, algorithm="HS256")

def test_user_registration():
    """Test user registration endpoint"""
    print("🧪 Testing user registration...")
    
    # Use timestamp to make email unique
    import time
    timestamp = int(time.time())
    
    user_data = {
        "email": f"testuser{timestamp}@example.com",
        "username": f"testuser{timestamp}",
        "password": "securepassword",
        "profile": {
            "first_name": "Test",
            "last_name": "User",
            "display_name": "TestUser",
            "bio": "I'm a test user for the Kilter Board app",
            "preferred_units": "metric",
            "privacy_settings": {
                "profile_visibility": "public",
                "statistics_visibility": "public",
                "history_visibility": "public"
            }
        }
    }
    
    response = requests.post(f"{BASE_URL}/users", json=user_data)
    
    if response.status_code == 200:
        result = response.json()
        print(f"✅ User registered successfully: {result['user']['username']}")
        return {
            'id': result['user']['id'],
            'email': result['user']['email'],
            'username': result['user']['username']
        }
    else:
        print(f"❌ Registration failed: {response.status_code} - {response.text}")
        return None

def test_get_current_user(user_id: str, email: str, username: str):
    """Test getting current user profile"""
    print("🧪 Testing get current user...")
    
    # Create JWT token for the test user
    token = create_test_jwt(user_id, email, username)
    headers = {"Authorization": f"Bearer {token}"}
    
    response = requests.get(f"{BASE_URL}/users/me", headers=headers)
    
    if response.status_code == 200:
        result = response.json()
        print(f"✅ Got user profile: {result['username']}")
        print(f"   Profile: {result['profile']['display_name']}")
        print(f"   Statistics: {result['statistics']}")
        return True
    else:
        print(f"❌ Get user failed: {response.status_code} - {response.text}")
        return False

def test_update_user_profile(user_id: str, email: str, username: str):
    """Test updating user profile"""
    print("🧪 Testing profile update...")
    
    token = create_test_jwt(user_id, email, username)
    headers = {"Authorization": f"Bearer {token}"}
    
    update_data = {
        "profile": {
            "first_name": "Updated",
            "last_name": "User",
            "display_name": "UpdatedTestUser",
            "bio": "Updated bio for testing",
            "location": "Test City",
            "preferred_units": "imperial",
            "privacy_settings": {
                "profile_visibility": "public",
                "statistics_visibility": "public",
                "history_visibility": "friends"
            }
        }
    }
    
    response = requests.put(f"{BASE_URL}/users/me", json=update_data, headers=headers)
    
    if response.status_code == 200:
        print("✅ Profile updated successfully")
        return True
    else:
        print(f"❌ Profile update failed: {response.status_code} - {response.text}")
        return False

def test_get_user_by_id(user_id: str):
    """Test getting public user profile by ID"""
    print("🧪 Testing get user by ID...")
    
    response = requests.get(f"{BASE_URL}/users/{user_id}")
    
    if response.status_code == 200:
        result = response.json()
        print(f"✅ Got public profile: {result['username']}")
        print(f"   Display name: {result['profile']['display_name']}")
        return True
    else:
        print(f"❌ Get user by ID failed: {response.status_code} - {response.text}")
        return False

def test_delete_user(user_id: str, email: str, username: str):
    """Test user account deletion"""
    print("🧪 Testing account deletion...")
    
    token = create_test_jwt(user_id, email, username)
    headers = {"Authorization": f"Bearer {token}"}
    
    response = requests.delete(f"{BASE_URL}/users/me", headers=headers)
    
    if response.status_code == 200:
        print("✅ Account deleted successfully")
        return True
    else:
        print(f"❌ Account deletion failed: {response.status_code} - {response.text}")
        return False

def test_health_check():
    """Test that the server is running"""
    print("🧪 Testing server health...")
    
    try:
        response = requests.get(f"{BASE_URL}/health", timeout=5)
        if response.status_code == 200:
            print("✅ Server is healthy")
            return True
        else:
            print(f"❌ Health check failed: {response.status_code}")
            return False
    except requests.exceptions.RequestException as e:
        print(f"❌ Cannot connect to server: {e}")
        return False

def main():
    """Run all user API tests"""
    print("🚀 Starting User Management API Tests")
    print("=" * 50)
    
    # Check if server is running
    if not test_health_check():
        print("\n❌ Server is not running. Please start the backend server first.")
        print("Run: cd backend && cargo run")
        sys.exit(1)
    
    print()
    
    # Test user registration
    user_data = test_user_registration()
    if not user_data:
        print("\n❌ Cannot continue without successful registration")
        sys.exit(1)
    
    user_id = user_data['id']
    email = user_data['email']
    username = user_data['username']
    
    print()
    
    # Test getting current user
    if not test_get_current_user(user_id, email, username):
        print("\n❌ Get current user test failed")
    
    print()
    
    # Test profile update
    if not test_update_user_profile(user_id, email, username):
        print("\n❌ Profile update test failed")
    
    print()
    
    # Test getting user by ID (public profile)
    if not test_get_user_by_id(user_id):
        print("\n❌ Get user by ID test failed")
    
    print()
    
    # Test account deletion (do this last)
    if not test_delete_user(user_id, email, username):
        print("\n❌ Account deletion test failed")
    
    print()
    print("=" * 50)
    print("🎉 User Management API Tests Complete!")

if __name__ == "__main__":
    main()
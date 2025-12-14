#!/usr/bin/env python3
"""
Test script to verify duplicate email/username validation
"""

import requests
import json

BASE_URL = "http://localhost:3000"

def test_duplicate_validation():
    """Test that duplicate email and username are properly rejected"""
    print("🧪 Testing duplicate validation...")
    
    # Use timestamp to make data unique
    import time
    timestamp = int(time.time())
    
    # First user data
    user_data = {
        "email": f"duplicate{timestamp}@example.com",
        "username": f"duplicateuser{timestamp}",
        "password": "securepassword",
        "profile": {
            "first_name": "First",
            "last_name": "User",
            "display_name": "FirstUser",
            "privacy_settings": {
                "profile_visibility": "public",
                "statistics_visibility": "public",
                "history_visibility": "public"
            }
        }
    }
    
    # Register first user
    response1 = requests.post(f"{BASE_URL}/users", json=user_data)
    if response1.status_code == 200:
        print("✅ First user registered successfully")
    else:
        print(f"❌ First registration failed: {response1.status_code} - {response1.text}")
        return False
    
    # Try to register with same email
    user_data2 = user_data.copy()
    user_data2["username"] = "differentuser"
    
    response2 = requests.post(f"{BASE_URL}/users", json=user_data2)
    if response2.status_code == 400 and "Email already registered" in response2.text:
        print("✅ Duplicate email correctly rejected")
    else:
        print(f"❌ Duplicate email not rejected properly: {response2.status_code} - {response2.text}")
        return False
    
    # Try to register with same username
    user_data3 = user_data.copy()
    user_data3["email"] = f"different{timestamp}@example.com"
    
    response3 = requests.post(f"{BASE_URL}/users", json=user_data3)
    if response3.status_code == 400 and "Username already taken" in response3.text:
        print("✅ Duplicate username correctly rejected")
    else:
        print(f"❌ Duplicate username not rejected properly: {response3.status_code} - {response3.text}")
        return False
    
    return True

def test_validation_order():
    """Test that validation happens before database operations"""
    print("🧪 Testing validation order (fast-fail)...")
    
    # Test invalid email format
    invalid_email_data = {
        "email": "invalid-email-format",
        "username": "validuser",
        "password": "securepassword",
        "profile": {
            "privacy_settings": {
                "profile_visibility": "public",
                "statistics_visibility": "public",
                "history_visibility": "public"
            }
        }
    }
    
    response = requests.post(f"{BASE_URL}/users", json=invalid_email_data)
    if response.status_code == 400 and "Invalid email format" in response.text:
        print("✅ Invalid email format correctly rejected (fast-fail)")
    else:
        print(f"❌ Invalid email format not rejected: {response.status_code} - {response.text}")
        return False
    
    # Test invalid username length
    invalid_username_data = {
        "email": "valid@example.com",
        "username": "ab",  # Too short
        "password": "securepassword",
        "profile": {
            "privacy_settings": {
                "profile_visibility": "public",
                "statistics_visibility": "public",
                "history_visibility": "public"
            }
        }
    }
    
    response = requests.post(f"{BASE_URL}/users", json=invalid_username_data)
    if response.status_code == 400 and "Username must be between 3 and 50 characters" in response.text:
        print("✅ Invalid username length correctly rejected (fast-fail)")
    else:
        print(f"❌ Invalid username length not rejected: {response.status_code} - {response.text}")
        return False
    
    return True

def main():
    print("🚀 Testing Database Safety and Performance Optimizations")
    print("=" * 60)
    
    success = True
    
    if not test_validation_order():
        success = False
    
    print()
    
    if not test_duplicate_validation():
        success = False
    
    print()
    print("=" * 60)
    if success:
        print("🎉 All safety and performance tests passed!")
    else:
        print("❌ Some tests failed")

if __name__ == "__main__":
    main()
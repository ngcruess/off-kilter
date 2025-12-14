#!/usr/bin/env python3
"""
Test script to generate JWT tokens and test authentication endpoints
"""
import jwt
import uuid
import time
from datetime import datetime, timedelta
import requests
import json

# JWT configuration (should match backend)
JWT_SECRET = "your-super-secret-jwt-key-change-this-in-production-please"
JWT_ALGORITHM = "HS256"
EXPIRATION_HOURS = 24

def create_test_token(user_id=None, email="test@example.com", username="testuser"):
    """Create a test JWT token"""
    if user_id is None:
        user_id = str(uuid.uuid4())
    
    now = datetime.utcnow()
    exp = now + timedelta(hours=EXPIRATION_HOURS)
    
    payload = {
        "sub": user_id,
        "email": email,
        "username": username,
        "exp": int(exp.timestamp()),
        "iat": int(now.timestamp())
    }
    
    token = jwt.encode(payload, JWT_SECRET, algorithm=JWT_ALGORITHM)
    return token, user_id

def test_endpoints():
    """Test the authentication endpoints"""
    base_url = "http://localhost:3000"
    
    # Create a test token
    token, user_id = create_test_token()
    headers = {"Authorization": f"Bearer {token}"}
    
    print(f"Generated test token for user: {user_id}")
    print(f"Token: {token[:50]}...")
    print()
    
    # Test public endpoint
    print("Testing public endpoint (/health):")
    try:
        response = requests.get(f"{base_url}/health")
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
    except Exception as e:
        print(f"Error: {e}")
    print()
    
    # Test protected endpoint without token
    print("Testing protected endpoint without token (/protected):")
    try:
        response = requests.get(f"{base_url}/protected")
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
    except Exception as e:
        print(f"Error: {e}")
    print()
    
    # Test protected endpoint with token
    print("Testing protected endpoint with token (/protected):")
    try:
        response = requests.get(f"{base_url}/protected", headers=headers)
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
    except Exception as e:
        print(f"Error: {e}")
    print()
    
    # Test user info endpoint with token
    print("Testing user info endpoint with token (/user-info):")
    try:
        response = requests.get(f"{base_url}/user-info", headers=headers)
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
    except Exception as e:
        print(f"Error: {e}")
    print()
    
    # Test with invalid token
    print("Testing with invalid token:")
    invalid_headers = {"Authorization": "Bearer invalid-token"}
    try:
        response = requests.get(f"{base_url}/protected", headers=invalid_headers)
        print(f"Status: {response.status_code}")
        print(f"Response: {response.json()}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    test_endpoints()
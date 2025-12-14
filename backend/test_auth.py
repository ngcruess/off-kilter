#!/usr/bin/env python3
"""
Test script for authentication middleware
Creates a JWT token and tests protected endpoints
"""

import jwt
import requests
import json
from datetime import datetime, timedelta
import uuid

# JWT configuration (matches backend .env)
JWT_SECRET = "your-super-secret-jwt-key-change-this-in-production-please"
ALGORITHM = "HS256"

# Backend URL
BASE_URL = "http://localhost:3000"

def create_test_token():
    """Create a test JWT token"""
    user_id = str(uuid.uuid4())
    now = datetime.utcnow()
    exp = now + timedelta(hours=24)
    
    payload = {
        "sub": user_id,
        "email": "test@example.com",
        "username": "testuser",
        "exp": int(exp.timestamp()),
        "iat": int(now.timestamp())
    }
    
    token = jwt.encode(payload, JWT_SECRET, algorithm=ALGORITHM)
    return token, user_id

def test_endpoints():
    """Test various endpoints with and without authentication"""
    
    print("🧪 Testing Kilter Board Authentication Middleware")
    print("=" * 50)
    
    # Test public endpoint
    print("\n1. Testing public endpoint (no auth required)")
    try:
        response = requests.get(f"{BASE_URL}/")
        print(f"   Status: {response.status_code}")
        print(f"   Response: {response.text}")
    except requests.exceptions.ConnectionError:
        print("   ❌ Server not running. Start with: just dev")
        return
    
    # Test health endpoint
    print("\n2. Testing health endpoint (no auth required)")
    response = requests.get(f"{BASE_URL}/health")
    print(f"   Status: {response.status_code}")
    if response.status_code == 200:
        print(f"   Response: {response.json()}")
    
    # Test protected endpoint without token
    print("\n3. Testing protected endpoint without token")
    response = requests.get(f"{BASE_URL}/protected")
    print(f"   Status: {response.status_code}")
    if response.status_code == 401:
        print(f"   Response: {response.json()}")
        print("   ✅ Correctly rejected unauthenticated request")
    
    # Create test token
    print("\n4. Creating test JWT token")
    token, user_id = create_test_token()
    print(f"   User ID: {user_id}")
    print(f"   Token: {token[:50]}...")
    
    # Test protected endpoint with token
    print("\n5. Testing protected endpoint with valid token")
    headers = {"Authorization": f"Bearer {token}"}
    response = requests.get(f"{BASE_URL}/protected", headers=headers)
    print(f"   Status: {response.status_code}")
    if response.status_code == 200:
        print(f"   Response: {response.json()}")
        print("   ✅ Successfully authenticated")
    
    # Test user info endpoint
    print("\n6. Testing user info extraction endpoint")
    response = requests.get(f"{BASE_URL}/user-info", headers=headers)
    print(f"   Status: {response.status_code}")
    if response.status_code == 200:
        data = response.json()
        print(f"   Response: {json.dumps(data, indent=2)}")
        print("   ✅ Successfully extracted user info from JWT")
    
    # Test with invalid token
    print("\n7. Testing with invalid token")
    invalid_headers = {"Authorization": "Bearer invalid-token-here"}
    response = requests.get(f"{BASE_URL}/protected", headers=invalid_headers)
    print(f"   Status: {response.status_code}")
    if response.status_code == 401:
        print(f"   Response: {response.json()}")
        print("   ✅ Correctly rejected invalid token")
    
    print("\n" + "=" * 50)
    print("🎉 Authentication middleware tests completed!")

if __name__ == "__main__":
    test_endpoints()
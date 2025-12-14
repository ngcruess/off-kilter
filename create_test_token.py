#!/usr/bin/env python3
import jwt
import uuid
from datetime import datetime, timedelta

# JWT configuration (matching the Rust backend)
secret = "your-super-secret-jwt-key-change-this-in-production-please"
algorithm = "HS256"

# Create test claims
user_id = str(uuid.uuid4())
now = datetime.utcnow()
exp = now + timedelta(hours=24)

claims = {
    "sub": user_id,
    "email": "test@example.com",
    "username": "testuser",
    "exp": int(exp.timestamp()),
    "iat": int(now.timestamp())
}

# Create token
token = jwt.encode(claims, secret, algorithm=algorithm)

print(f"Test JWT Token:")
print(token)
print(f"\nUser ID: {user_id}")
print(f"Email: {claims['email']}")
print(f"Username: {claims['username']}")
print(f"\nTest command:")
print(f'curl -X GET http://localhost:3000/user-info -H "Authorization: Bearer {token}"')
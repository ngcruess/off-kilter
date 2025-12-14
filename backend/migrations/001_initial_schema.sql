-- Initial database schema for Kilter Board application
-- This migration creates the foundational tables

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for users
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);

-- User profiles (JSON storage for flexibility)
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    profile_data JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User climbing statistics
CREATE TABLE user_statistics (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    total_attempts INTEGER DEFAULT 0,
    total_ascents INTEGER DEFAULT 0,
    personal_best_grade VARCHAR(10),
    statistics_data JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Boulder problems
CREATE TABLE boulder_problems (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    creator_id UUID NOT NULL REFERENCES users(id),
    difficulty VARCHAR(10) NOT NULL, -- V-scale grade
    hold_configuration JSONB NOT NULL,
    tags TEXT[] DEFAULT '{}',
    ascent_count INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for boulder problems
CREATE INDEX idx_boulder_problems_creator_id ON boulder_problems(creator_id);
CREATE INDEX idx_boulder_problems_difficulty ON boulder_problems(difficulty);
CREATE INDEX idx_boulder_problems_tags ON boulder_problems USING GIN(tags);
CREATE INDEX idx_boulder_problems_created_at ON boulder_problems(created_at);
CREATE INDEX idx_boulder_problems_ascent_count ON boulder_problems(ascent_count);
CREATE INDEX idx_boulder_problems_published ON boulder_problems(is_published);

-- Votes table (dual-dimension: stars + difficulty)
CREATE TABLE votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id UUID NOT NULL REFERENCES boulder_problems(id) ON DELETE CASCADE,
    star_rating INTEGER NOT NULL CHECK (star_rating >= 1 AND star_rating <= 4),
    difficulty_grade VARCHAR(10) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, problem_id)
);

-- Create indexes for votes
CREATE INDEX idx_votes_user_id ON votes(user_id);
CREATE INDEX idx_votes_problem_id ON votes(problem_id);
CREATE INDEX idx_votes_star_rating ON votes(star_rating);
CREATE INDEX idx_votes_difficulty_grade ON votes(difficulty_grade);

-- User attempts table
CREATE TABLE user_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id UUID NOT NULL REFERENCES boulder_problems(id) ON DELETE CASCADE,
    outcome VARCHAR(20) NOT NULL CHECK (outcome IN ('attempt', 'ascent')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for user attempts
CREATE INDEX idx_user_attempts_user_id ON user_attempts(user_id);
CREATE INDEX idx_user_attempts_problem_id ON user_attempts(problem_id);
CREATE INDEX idx_user_attempts_outcome ON user_attempts(outcome);
CREATE INDEX idx_user_attempts_created_at ON user_attempts(created_at);

-- Problem sets table
CREATE TABLE problem_sets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    creator_id UUID NOT NULL REFERENCES users(id),
    problem_ids UUID[] NOT NULL DEFAULT '{}',
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for problem sets
CREATE INDEX idx_problem_sets_creator_id ON problem_sets(creator_id);
CREATE INDEX idx_problem_sets_published ON problem_sets(is_published);
CREATE INDEX idx_problem_sets_created_at ON problem_sets(created_at);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON user_profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_statistics_updated_at BEFORE UPDATE ON user_statistics
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_boulder_problems_updated_at BEFORE UPDATE ON boulder_problems
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_votes_updated_at BEFORE UPDATE ON votes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_problem_sets_updated_at BEFORE UPDATE ON problem_sets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
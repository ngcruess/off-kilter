// Core data types for the mobile application

export interface UserAccount {
  id: string;
  email: string;
  username: string;
  profile: UserProfile;
  statistics: ClimbingStatistics;
  createdAt: Date;
  updatedAt: Date;
}

export interface UserProfile {
  firstName?: string;
  lastName?: string;
  avatar?: string;
  bio?: string;
}

export interface ClimbingStatistics {
  totalAttempts: number;
  totalAscents: number;
  personalBestGrade: string;
  streakRecords: StreakRecord[];
  milestones: Milestone[];
}

export interface StreakRecord {
  type: 'daily' | 'weekly' | 'monthly';
  count: number;
  startDate: Date;
  endDate: Date;
}

export interface Milestone {
  id: string;
  name: string;
  description: string;
  achievedAt: Date;
}

export interface BoulderProblem {
  id: string;
  name: string;
  creatorId: string;
  holdConfiguration: HoldConfiguration;
  difficulty: string;
  tags: ProblemTag[];
  createdAt: Date;
  updatedAt: Date;
}

export interface HoldConfiguration {
  holds: Hold[];
  boardModel: string;
  version: string;
}

export interface Hold {
  id: string;
  position: {
    x: number;
    y: number;
  };
  type: 'start' | 'middle' | 'finish';
}

export interface ProblemTag {
  id: string;
  name: string;
  category: TagCategory;
}

export type TagCategory = 'style' | 'difficulty' | 'feature' | 'body-position';

export interface Vote {
  id: string;
  userId: string;
  problemId: string;
  starRating: number; // 1-4 stars
  difficultyGrade: string; // V-scale
  createdAt: Date;
  updatedAt: Date;
}

export interface AggregateRating {
  problemId: string;
  averageStars: number;
  consensusDifficulty: string;
  totalVotes: number;
  starDistribution: number[];
  gradeDistribution: Map<string, number>;
}
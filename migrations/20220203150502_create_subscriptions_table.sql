-- migrations/${timestamp}_create_subscriptions_)table.sql
-- Create subscriptions table
CREATE TABLE IF NOT EXISTS subscriptions (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz NOT NULL DEFAULT now()
);
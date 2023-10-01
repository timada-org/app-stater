-- Add down migration script here

DROP TABLE IF EXISTS evento_events;
DROP TABLE IF EXISTS evento_deadletters;
DROP TABLE IF EXISTS evento_subscriptions;

DROP TABLE IF EXISTS feed_feeds;
DROP TABLE IF EXISTS feed_tags_count;

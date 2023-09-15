-- Add down migration script here

DROP TABLE IF EXISTS _evento_events;
DROP TABLE IF EXISTS _evento_deadletters;
DROP TABLE IF EXISTS _evento_subscriptions;

DROP TABLE IF EXISTS feed_feeds;
DROP TABLE IF EXISTS feed_tags_count;
CREATE TABLE IF NOT EXISTS feed_feeds
(
    id VARCHAR(26) NOT NULL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    author VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    content_short VARCHAR(250) NOT NULL,
    total_likes int4 NOT NULL DEFAULT 0,
    tags VARCHAR(30)[] NOT NULL,
    user_id UUID NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX ON feed_feeds (user_id);
CREATE INDEX ON feed_feeds USING GIN (tags array_ops);

CREATE TABLE IF NOT EXISTS feed_tags_count
(
    tag VARCHAR(30) NOT NULL PRIMARY KEY,
    total_count int4 NOT NULL DEFAULT 0
);

CREATE INDEX ON feed_tags_count (total_count);

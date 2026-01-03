-- sql/schema.sql

CREATE TABLE performance (
    id INTEGER PRIMARY KEY,
    service TEXT NOT NULL,
    metric TEXT NOT NULL,
    value REAL NOT NULL,
    recorded_at TEXT NOT NULL  -- ISO-8601 datetime
);

CREATE INDEX idx_performance_service ON performance(service);
CREATE INDEX idx_performance_time ON performance(recorded_at);

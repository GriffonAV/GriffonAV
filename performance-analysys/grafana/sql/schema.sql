CREATE TABLE metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service TEXT NOT NULL,
    metric TEXT NOT NULL,
    value REAL NOT NULL,
    ts TEXT NOT NULL   -- RFC3339 timestamp
);

CREATE INDEX idx_metrics_ts ON metrics(ts);
CREATE INDEX idx_metrics_service_metric ON metrics(service, metric);

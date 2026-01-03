-- sql/seed.sql

INSERT INTO performance (service, metric, value, recorded_at) VALUES
('api', 'latency_ms', 120.5, '2026-01-01 10:00:00'),
('api', 'latency_ms', 98.2,  '2026-01-01 11:00:00'),
('web', 'error_rate', 0.02,  '2026-01-01 10:00:00');

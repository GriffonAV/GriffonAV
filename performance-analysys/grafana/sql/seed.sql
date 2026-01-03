INSERT INTO metrics (service, metric, value, ts) VALUES
-- API service
('api', 'latency_ms', 120, '2026-01-01 00:00:00'),
('api', 'latency_ms', 135, '2026-01-01 01:00:00'),
('api', 'latency_ms', 110, '2026-01-01 02:00:00'),
('api', 'error_rate', 0.01, '2026-01-01 00:00:00'),
('api', 'error_rate', 0.02, '2026-01-01 01:00:00'),
('api', 'error_rate', 0.015, '2026-01-01 02:00:00'),
('api', 'throughput_rps', 320, '2026-01-01 00:00:00'),
('api', 'throughput_rps', 350, '2026-01-01 01:00:00'),
('api', 'throughput_rps', 340, '2026-01-01 02:00:00'),

-- Web service
('web', 'latency_ms', 220, '2026-01-01 00:00:00'),
('web', 'latency_ms', 210, '2026-01-01 01:00:00'),
('web', 'latency_ms', 230, '2026-01-01 02:00:00'),
('web', 'error_rate', 0.03, '2026-01-01 00:00:00'),
('web', 'error_rate', 0.025, '2026-01-01 01:00:00'),
('web', 'error_rate', 0.04, '2026-01-01 02:00:00'),
('web', 'throughput_rps', 150, '2026-01-01 00:00:00'),
('web', 'throughput_rps', 165, '2026-01-01 01:00:00'),
('web', 'throughput_rps', 140, '2026-01-01 02:00:00'),

-- Worker service
('worker', 'latency_ms', 80, '2026-01-01 00:00:00'),
('worker', 'latency_ms', 90, '2026-01-01 01:00:00'),
('worker', 'latency_ms', 85, '2026-01-01 02:00:00'),
('worker', 'error_rate', 0.005, '2026-01-01 00:00:00'),
('worker', 'error_rate', 0.0, '2026-01-01 01:00:00'),
('worker', 'error_rate', 0.01, '2026-01-01 02:00:00'),
('worker', 'throughput_rps', 500, '2026-01-01 00:00:00'),
('worker', 'throughput_rps', 520, '2026-01-01 01:00:00'),
('worker', 'throughput_rps', 510, '2026-01-01 02:00:00');

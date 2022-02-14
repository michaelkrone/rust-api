CREATE TABLE measurements (
  id BIGSERIAL PRIMARY KEY,
  val FLOAT8 NOT NULL,
  typ VARCHAR(4) NOT NULL,
  node_id INTEGER NOT NULL,
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_measurements_node_id ON measurements(node_id);

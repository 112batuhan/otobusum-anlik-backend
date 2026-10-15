CREATE TABLE IF NOT EXISTS lines(
  id SERIAL PRIMARY KEY,
  code TEXT NOT NULL,
  title TEXT NOT NULL,
  city TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS id_code_index_lines ON lines (code, city);

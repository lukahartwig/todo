CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
  title       TEXT NOT NULL,
  status      TEXT NOT NULL DEFAULT 'READY'
);

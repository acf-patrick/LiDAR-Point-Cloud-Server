DROP TABLE files;
CREATE TABLE files(
  id TEXT NOT NULL PRIMARY KEY,
  file_source_id INTEGER NOT NULL,
  version_minor INTEGER NOT NULL,
  version_major INTEGER NOT NULL,
  date TEXT,
  has_gps_time INTEGER NOT NULL,
  has_color INTEGER NOT NULL,
  is_compressed INTEGER NOT NULL,
  scale_x REAL NOT NULL,
  scale_y REAL NOT NULL,
  scale_z REAL NOT NULL,
  offset_x REAL NOT NULL,
  offset_y REAL NOT NULL,
  offset_z REAL NOT NULL,
  min_x REAL NOT NULL,
  min_y REAL NOT NULL,
  min_z REAL NOT NULL,
  max_x REAL NOT NULL,
  max_y REAL NOT NULL,
  max_z REAL NOT NULL,
  number_of_points BIGINT NOT NULL
);

DROP TABLE "file";

CREATE TABLE parts(
  id TEXT NOT NULL PRIMARY KEY,
  file_id TEXT NOT NULL,
  x REAL NOT NULL,
  y REAL NOT NULL,
  z REAL NOT NULL,
  edge REAL NOT NULL,
  FOREIGN KEY (file_id) REFERENCES files(id)
)
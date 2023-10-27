DROP TABLE parts;

CREATE TABLE file(
  id TEXT NOT NULL PRIMARY KEY,
  file_source_id INTEGER,
  version_minor INTEGER,
  version_major INTEGER,
  date TEXT,
  has_gps_time INTEGER,
  has_color INTEGER,
  is_compressed INTEGER,
  scale_x REAL,
  scale_y REAL,
  scale_z REAL,
  offset_x REAL,
  offset_y REAL,
  offset_z REAL,
  min_x REAL,
  min_y REAL,
  min_z REAL,
  max_x REAL,
  max_y REAL,
  max_z REAL,
  number_of_points INTEGER
);

DROP TABLE files;
CREATE TABLE files(
  id TEXT NOT NULL PRIMARY KEY,
  file_id TEXT NOT NULL,
  x REAL NOT NULL,
  y REAL NOT NULL,
  z REAL NOT NULL,
  edge REAL NOT NULL
)
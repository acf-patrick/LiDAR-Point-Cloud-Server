-- This file should undo anything in `up.sql`
CREATE TABLE files (
  id VARCHAR(255) PRIMARY KEY,
  path VARCHAR(255) NOT NULL
);

ALTER TABLE parts
ADD CONSTRAINT fk_file
FOREIGN KEY (file_id)
REFERENCES files(id);
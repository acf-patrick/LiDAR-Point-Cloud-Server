-- Your SQL goes here
ALTER TABLE "parts" 
ALTER COLUMN "x" SET NOT NULL,
ALTER COLUMN "y" SET NOT NULL,
ALTER COLUMN "z" SET NOT NULL,
ALTER COLUMN "edge" SET NOT NULL;
-- This file should undo anything in `up.sql`
ALTER TABLE "parts" 
ALTER COLUMN "x" DROP NOT NULL,
ALTER COLUMN "y" DROP NOT NULL,
ALTER COLUMN "z" DROP NOT NULL,
ALTER COLUMN "edge" DROP NOT NULL;
-- This file should undo anything in `up.sql`
ALTER "files"
ALTER COLUMN "path" DROP NOT NULL;
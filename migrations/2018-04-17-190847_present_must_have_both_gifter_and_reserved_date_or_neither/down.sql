-- This file should undo anything in `up.sql`

ALTER TABLE presents
  DROP CONSTRAINT gifter_and_reserved;

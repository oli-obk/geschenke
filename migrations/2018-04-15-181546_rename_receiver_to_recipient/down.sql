-- This file should undo anything in `up.sql`

ALTER TABLE geschenke
  RENAME COLUMN recipient TO receiver;
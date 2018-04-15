-- This file should undo anything in `up.sql`

ALTER TABLE geschenke
  RENAME COLUMN reserved_date TO obtained_date;

COMMENT ON COLUMN geschenke.obtained_date IS 'the date the present has been obtained (bought, made, ...)';

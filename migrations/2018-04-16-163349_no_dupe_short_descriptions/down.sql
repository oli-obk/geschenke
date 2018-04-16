-- This file should undo anything in `up.sql`

ALTER TABLE presents
  REMOVE CONSTRAINT no_dups_present_short_descriptions;

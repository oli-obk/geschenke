-- Your SQL goes here

ALTER TABLE presents
  ADD CONSTRAINT no_dups_present_short_descriptions UNIQUE (short_description, recipient);

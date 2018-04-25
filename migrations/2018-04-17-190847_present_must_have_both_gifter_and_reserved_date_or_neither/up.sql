-- Your SQL goes here

ALTER TABLE presents
  ADD CONSTRAINT gifter_and_reserved CHECK (gifter IS NULL = (reserved_date IS NULL));

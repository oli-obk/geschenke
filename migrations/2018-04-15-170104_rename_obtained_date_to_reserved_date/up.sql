-- Your SQL goes here

ALTER TABLE geschenke
  RENAME COLUMN obtained_date TO reserved_date;

COMMENT ON COLUMN geschenke.reserved_date IS 'the date the present has been reserved by a user';

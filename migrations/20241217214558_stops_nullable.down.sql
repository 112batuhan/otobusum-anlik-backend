-- Ensure no NULL values before setting NOT NULL constraints
UPDATE stops SET province = 'default_value' WHERE province IS NULL;
UPDATE stops SET smart = false WHERE smart IS NULL;
UPDATE stops SET physical = false WHERE physical IS NULL;
UPDATE stops SET stop_type = 'default_value' WHERE stop_type IS NULL;
UPDATE stops SET disabled_can_use = false WHERE disabled_can_use IS NULL;

-- Reapply NOT NULL constraints
ALTER TABLE stops ALTER COLUMN province SET NOT NULL;
ALTER TABLE stops ALTER COLUMN smart SET NOT NULL;
ALTER TABLE stops ALTER COLUMN physical SET NOT NULL;
ALTER TABLE stops ALTER COLUMN stop_type SET NOT NULL;
ALTER TABLE stops ALTER COLUMN disabled_can_use SET NOT NULL;

-- Restore the removed column
ALTER TABLE stops ADD COLUMN direction TEXT;

-- Remove route_code column and unique index from line_stops
ALTER TABLE line_stops DROP COLUMN route_code;
DROP INDEX IF EXISTS id_code_index_line_route_stops;;

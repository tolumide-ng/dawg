-- Add down migration script here
DROP TABLE node_edge;
DROP FUNCTION scan_rows;
DROP FUNCTION IF EXISTS allow_first_row_to_be_null;
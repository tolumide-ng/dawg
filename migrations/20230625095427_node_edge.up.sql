-- Add up migration script here
CREATE TABLE node_edge(
    node_id BIGINT NOT NULL, -- this is the parent node, the one linking to the edge
    edge_id BIGINT NOT NULL,
    FOREIGN KEY (node_id) REFERENCES node(id) ON DELETE CASCADE,
    FOREIGN KEY (edge_id) REFERENCES node(id) ON DELETE CASCADE
);



-- -- Allow the insertion of a trigger
-- CREATE OR REPLACE FUNCTION allow_null_trigger()
-- RETURNS TRIGGER AS $$
-- DECLARE
--     has_rows BOOLEAN;
-- BEGIN
--     SELECT EXISTS (SELECT 1 FROM node_edge) INTO  has_rows;
    
--     -- If no rows are found, execute the desired logic
--     IF NOT has_rows THEN
--         IF NEW.id = 1 THEN
--             NEW.edge_id = NULL;
--         END IF;
--     END IF;

--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- -- Create the trigger
-- CREATE TRIGGER allow_null
-- BEFORE INSERT OR UPDATE ON node_edge
-- FOR EACH ROW
-- EXECUTE FUNCTION allow_null_trigger();




CREATE OR REPLACE FUNCTION scan_rows(BIGINT, TEXT[])
    RETURNS TEXT[][] AS $$
    DECLARE
    char TEXT;
    curr_node INTEGER := $1; -- the id of the root node
    is_terminal_val BOOLEAN;
    results TEXT[][] := '{}';
    BEGIN
    FOREACH char IN ARRAY $2
    LOOP    
        SELECT n.id, n.terminal
        INTO curr_node, is_terminal_val
        FROM node_edge ne
        LEFT JOIN node n ON ne.edge_id = n.id
        WHERE ne.node_id = curr_node
        AND n.letter = char;

        IF curr_node IS NULL THEN
            EXIT;
        END IF;

        -- Append the result as an array to the results array
        results := results || ARRAY[[CAST(curr_node AS TEXT), CAST(is_terminal_val AS TEXT)]];
    END LOOP;

    -- Return the array of arrays as the result
    RETURN results;
END;
$$ LANGUAGE plpgsql;

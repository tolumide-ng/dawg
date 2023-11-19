use sqlx::{Pool, Postgres};

use crate::{
    dawg::sqlx_dawg::SqlSearchRes,
    node::sqlx_node::{NodeEdge, SqlNode},
};

pub(crate) struct Repository;

impl Repository {
    pub async fn create_node(
        pool: &Pool<Postgres>,
        letter: String,
        parent: Option<i64>,
        terminal: bool,
    ) -> Result<SqlNode, sqlx::Error> {
        match parent {
            Some(parent_id) => {
                let next_node: Result<SqlNode, sqlx::Error> = sqlx::query_as!(
                    SqlNode, r#"
                        WITH new_node AS (
                            INSERT INTO node (letter, terminal) VALUES ($1, $2) RETURNING *
                        ), parent_node AS (
                            INSERT INTO node_edge (node_id, edge_id) SELECT $3, id FROM new_node
                        )
                        SELECT
                            new_node.id,
                            new_node.letter,
                            new_node.count,
                            new_node.terminal,
                            COALESCE(ARRAY_REMOVE(ARRAY_AGG(node_edge.edge_id), NULL), '{}'::integer[]) AS "edges!: Vec<i64>"
                        FROM
                            new_node
                        LEFT JOIN
                            node_edge ON new_node.id = node_edge.node_id
                        GROUP BY
                            new_node.id, new_node.letter, new_node.count, new_node.terminal;
                    "#,
                    letter, terminal, parent_id
                )
                .fetch_one(pool)
                .await;

                next_node
            }
            None => {
                // root node
                let next_node: Result<SqlNode, sqlx::Error> = sqlx::query_as!(
                    SqlNode, r#"
                        WITH new_node AS (
                            INSERT INTO node (letter, terminal) VALUES ($1, $2) RETURNING *
                        )
                        SELECT
                            new_node.id,
                            new_node.letter,
                            new_node.count,
                            new_node.terminal,
                            COALESCE(ARRAY(SELECT node_edge.edge_id FROM node_edge WHERE new_node.id = node_edge.node_id), '{}'::integer[]) AS "edges!: Vec<i64>"
                        FROM
                            new_node;
                        
                        "#,
                    letter, terminal
                )
                .fetch_one(pool)
                .await;

                next_node
            }
        }
    }

    /// Fetches the edges of all the nodes in the array(ids) provided.
    /// This function maintains the order of the array it was provided with
    /// e.g if the input is [1, 6, 3] the output would be nodes with that order of id
    /// output ==>> [SqlNode{1, ..}, SqlNode{6, ..}, SqlNode{3, ..}]
    pub async fn get_nodes_with_edges(
        pool: &Pool<Postgres>,
        ids: &[i64],
    ) -> Result<Vec<SqlNode>, sqlx::Error> {
        sqlx::query_as!(
            SqlNode,
            r#"
                WITH filtered_nodes AS (
                    SELECT id, letter, count, terminal
                    FROM node
                    WHERE id = ANY($1)
                )
                SELECT 
                    fn.id, 
                    fn.letter, 
                    fn.count, 
                    fn.terminal,
                    CASE WHEN COUNT(ne.edge_id) = 0 THEN ARRAY[]::bigint[] ELSE ARRAY_AGG(DISTINCT ne.edge_id) END AS "edges!: Vec<i64>"
                FROM filtered_nodes AS fn
                LEFT JOIN node_edge AS ne ON fn.id = ne.node_id
                GROUP BY fn.id, fn.letter, fn.count, fn.terminal
                ORDER BY array_position($1, fn.id);
            "#,
            &ids[..],
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_node(
        pool: &Pool<Postgres>,
        letter: String,
    ) -> Result<Option<SqlNode>, sqlx::Error> {
        sqlx::query_as!(
            SqlNode,
            r#"
            SELECT n.id, n.letter, n.count, n.terminal, ARRAY_AGG(DISTINCT ne.edge_id) AS "edges!: Vec<i64>"
            FROM node AS n
            LEFT JOIN node_edge AS ne on n.id=ne.node_id
            WHERE letter = $1
            GROUP BY n.id, n.letter, n.count, n.terminal
        "#,
            letter
        )
        .fetch_optional(pool)
        .await
    }

    /// Adds the edges(children) of a node(parent) to the node_edge table
    pub async fn add_parent(
        pool: &Pool<Postgres>,
        node_edge: &NodeEdge,
    ) -> Result<NodeEdge, sqlx::Error> {
        let NodeEdge {
            node_id, edge_id, ..
        } = node_edge;

        sqlx::query_as!(
            NodeEdge,
            r#"
                WITH inserted_edge AS (
                    INSERT INTO node_edge (node_id, edge_id) 
                    VALUES ($1, $2)
                    RETURNING *
                )
                SELECT inserted_edge.node_id, inserted_edge.edge_id, n.letter AS "edge_letter!: String"
                FROM inserted_edge
                LEFT JOIN node AS n ON n.id = inserted_edge.edge_id;
            "#,
            node_id,
            edge_id
        )
        .fetch_one(pool)
        .await
    }

    /// Checks if the provided word is valid
    /// Returns something like Vec<Vec<(a, b)>>
    /// where a is of type String and represents the id of the letter (node)
    /// where b is of type String and cane be either true or false depending on whether that letter(node) is a terminal
    /// Vec<Vec<(1, false)>, Vec<(16, true)>> means that the word is a valid word because the last item in the vector has b = true
    /// Vec<Vec<(1, false)>, Vec<(16, true)>, Vec<(21, false)>> means that the word is not a valid word because the last item in the vector has b = false
    /// There might be instances where the vector returned is shorter than the actual "word vector" provided by the user. In such cases, the word is invalid
    /// this is owing to the fact that the postgresql loop could not find a corresponding child of the last letter checked that is equiavlent to the expected
    /// next letter in the word provided by the user
    pub async fn is_word(
        pool: &Pool<Postgres>,
        root_id: i64,
        word: &[String],
    ) -> Result<Vec<SqlSearchRes>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT ARRAY (SELECT UNNEST(scan_rows($1, $2)));
            "#,
            root_id,
            word
        )
        .fetch_one(pool)
        .await;

        if let Ok(record) = result {
            let values = record.array.unwrap();
            let target_vec_length = values.len() / 2;

            let mut search_res: Vec<SqlSearchRes> = Vec::with_capacity(target_vec_length);

            for index in 0..target_vec_length {
                let node_id_index = index * 2;
                let is_terminal_index = (index * 2) + 1;

                search_res.push(SqlSearchRes::new(
                    values[node_id_index].parse::<i64>().unwrap(),
                    values[is_terminal_index].parse::<bool>().unwrap(),
                ))
            }

            return Ok(search_res);
        }

        return Err(result.unwrap_err());
    }
}

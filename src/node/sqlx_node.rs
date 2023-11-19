use std::fmt::Display;

use serde::{Deserialize, Serialize};

use sqlx::{self, postgres::PgRow, FromRow, Pool, Postgres, Row};

type SqlNodeID = i64;

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[sqlx(type_name = "lingo", rename_all = "lowercase")]
pub struct SqlNode {
    // the text/letter of this node === PLANS TO CHANGE THIS TO CHAR DEPENDING ON THE LANGUAGE? (or not depending)
    pub(crate) letter: String,
    // the unique id of this node
    pub(crate) id: SqlNodeID,
    // How many terminals (are formed)/extend from node
    pub(crate) count: i64,
    // is this node a terminal?
    pub terminal: bool,
    /// the letter
    /// parent: Option<Uuid>,
    /// children of this node >>>>>>>> i.e nodes that extend from this node
    /// #[sqlx(try_from = "i64")]
    /// this MUST not be accessble from outside the create (use get_edges instead), as this
    /// is only used for building the daeg
    pub(crate) edges: Vec<SqlNodeID>,
}

impl FromRow<'_, PgRow> for SqlNode {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let edges = if let Ok(value) = row.try_get("edges") {
            value
        } else {
            Vec::with_capacity(0)
        };

        Ok(Self {
            letter: row.try_get("letter")?,
            id: if let Ok(v) = row.try_get("id") { v } else { 0 },
            count: if let Ok(v) = row.try_get("count") {
                v
            } else {
                0
            },
            terminal: if let Ok(v) = row.try_get("terminal") {
                v
            } else {
                false
            },
            edges,
        })
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NodeEdge {
    /// usually the parent id
    pub node_id: SqlNodeID,
    // usually a child of the parent id
    pub edge_id: SqlNodeID,
    /// the corresponding letter of the edge_id (this is not required, but should be provided where possible)
    pub edge_letter: String,
}

impl NodeEdge {
    pub(crate) fn new(node_id: SqlNodeID, edge_id: SqlNodeID, edge_letter: String) -> Self {
        Self {
            node_id,
            edge_id,
            edge_letter,
        }
    }
}

impl SqlNode {
    pub(crate) fn new(id: SqlNodeID, letter: String, terminal: bool) -> Self {
        Self {
            id,
            terminal,
            count: 0,
            letter,
            edges: vec![],
            // parent: None,
        }
    }

    /// Returns the total of word terminals resulting from this node
    pub(crate) async fn num_reachable(&mut self) -> i64 {
        // this method is unimplemented
        // unimplemented!();

        if self.count != 0 {
            return self.count;
        }

        let mut count = 0;

        if self.terminal {
            count += 1;
        }

        // I would have to come back to this as it is not yet perfect yet: try using a small dictionary of words
        let all_children = sqlx::query!(
            r#"
            WITH RECURSIVE edges AS (
                    SELECT
                        id, letter, count, terminal
                    FROM
                        node
                    WHERE 
                        id = $1
                    UNION
                        SELECT
                            n.id,
                            n.letter,
                            n.count,
                            n.terminal
                        FROM
                            node n
                        INNER JOIN node_edge ON node_edge.node_id = n.id
                        WHERE n.count > 0
            ) SELECT * FROM edges;
        "#,
            self.id
        );

        // let edges = sqlx::query!(r#""#).await;

        0
    }

    /// Returns all the children(edges extending)  of(from) a node
    pub async fn get_edges(&self, pool: &Pool<Postgres>) -> Result<Vec<NodeEdge>, sqlx::Error> {
        sqlx::query_as!(
            NodeEdge,
            r#"
            SELECT ne.node_id, ne.edge_id, n.letter AS "edge_letter!: String"
            FROM node_edge AS ne LEFT JOIN node AS n on ne.edge_id = n.id
            WHERE node_id = $1
        "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }
}

// this can be implemented on the trait instead of on individual implementations
impl Display for SqlNode {
    /// always ensure that all the edges of the SqlNode are available before using this
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arr = vec![];

        if self.terminal {
            arr.push("1**".to_string());
        } else {
            arr.push("0**".to_string());
        }

        // use the user's desired feature flag to select which option to go with in this case
        for id in &self.edges {
            arr.push(id.to_string());
        }

        let name = arr.join("_");

        write!(f, "{}", name)
    }
}

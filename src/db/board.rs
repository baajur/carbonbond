use super::{get_pool, DBObject, ToFallible};
use crate::custom_error::{DataType, Fallible};

#[derive(Debug, Default)]
pub struct Board {
    pub id: i64,
    pub board_name: String,
    pub title: String,
    pub detail: String,
    pub ruling_party_id: i64,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl DBObject for Board {
    const TYPE: DataType = DataType::Board;
}

pub async fn get_by_name(name: &str) -> Fallible<Board> {
    let pool = get_pool();
    let user = sqlx::query_as!(Board, "SELECT * FROM boards WHERE board_name = $1", name)
        .fetch_one(pool)
        .await
        .to_fallible(name)?;
    Ok(user)
}

pub async fn create(board: &Board) -> Fallible<i64> {
    // TODO: 交易？
    let pool = get_pool();
    let board_id= sqlx::query!(
        "INSERT INTO boards (board_name, detail, title, ruling_party_id) VALUES ($1, $2, $3, $4) RETURNING id",
        board.board_name,
        board.detail,
        board.title,
        board.ruling_party_id
    )
    .fetch_one(pool)
    .await?
    .id;
    super::party::change_board(board.ruling_party_id, board_id).await?;
    Ok(board_id)
}

use juniper_from_schema::graphql_schema_from_file;
use juniper::ID;
use diesel::prelude::*;

use crate::db::{models as db_models, schema as db_schema};
use crate::custom_error::{Fallible, Error};
use crate::party;
use crate::forum;

use super::{id_to_i64, i64_to_id, Context, ContextTrait, Board, Article, Me, Party, systime_to_i32};

graphql_schema_from_file!("api/api.gql", error_type: Error, with_idents: [Query]);

pub struct Query;

impl QueryFields for Query {
    fn field_me(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Me, juniper_from_schema::Walked>,
    ) -> Fallible<Me> {
        match ex.context().get_id() {
            None => Ok(Me { id: None }),
            Some(id) => {
                if id == "".to_string() {
                    Ok(Me { id: None })
                } else {
                    Ok(Me { id: Some(id) })
                }
            }
        }
    }
    fn field_board(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Board, juniper_from_schema::Walked>,
        name: String,
    ) -> Fallible<Board> {
        let board = forum::get_board_by_name(&ex.context().get_pg_conn()?, &name)?;
        Ok(Board {
            id: i64_to_id(board.id),
            board_name: board.board_name,
            title: board.title,
            detail: board.detail,
            ruling_party_id: i64_to_id(board.ruling_party_id),
        })
    }
    fn field_article(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, juniper_from_schema::Walked>,
        id: ID,
    ) -> Fallible<Article> {
        use db_schema::articles::dsl;
        let article = dsl::articles
            .filter(dsl::id.eq(id_to_i64(&id)?))
            .first::<db_models::Article>(&ex.context().get_pg_conn()?)
            .map_err(|_| Error::new_logic("找不到文章", 404))?;
        Ok(Article {
            id: i64_to_id(article.id),
            title: article.title,
            board_id: i64_to_id(article.board_id),
            author_id: article.author_id,
            category_name: article.category_name,
            category_id: i64_to_id(article.category_id),
            create_time: systime_to_i32(article.create_time),
            energy: 0,
            root_id: i64_to_id(article.root_id),
        })
    }
    fn field_board_list(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Board, juniper_from_schema::Walked>,
        ids: Option<Vec<ID>>,
    ) -> Fallible<Vec<Board>> {
        use db_schema::boards::dsl::*;
        let mut query = boards.into_boxed();
        if let Some(ids) = ids {
            let ids: Fallible<Vec<i64>> = ids.iter().map(|t| id_to_i64(t)).collect();
            query = query.filter(id.eq_any(ids?));
        }
        let board_vec = query.load::<db_models::Board>(&ex.context().get_pg_conn()?)?;
        Ok(board_vec
            .into_iter()
            .map(|b| Board {
                id: i64_to_id(b.id),
                board_name: b.board_name,
                title: b.title,
                detail: b.detail,
                ruling_party_id: i64_to_id(b.ruling_party_id),
            })
            .collect())
    }
    fn field_article_list(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, juniper_from_schema::Walked>,
        board_name: String,
        offset: i32,
        page_size: i32,
        show_hidden: Option<bool>,
    ) -> Fallible<Vec<Article>> {
        let conn = ex.context().get_pg_conn()?;
        use db_schema::articles::dsl;
        let show_hidden = show_hidden.unwrap_or(false);
        let board = forum::get_board_by_name(&conn, &board_name)?;

        let mut query = dsl::articles
            .filter(dsl::board_id.eq(board.id))
            .into_boxed();
        if !show_hidden {
            query = query.filter(dsl::show_in_list.eq(true));
        }

        let article_vec = query
            .order(dsl::create_time.asc())
            .offset(offset as i64)
            .limit(page_size as i64)
            .load::<db_models::Article>(&conn)?;

        Ok(article_vec
            .into_iter()
            .map(|a| Article {
                id: i64_to_id(a.id),
                title: a.title.clone(),
                board_id: i64_to_id(a.board_id),
                author_id: a.author_id.clone(),
                category_id: i64_to_id(a.category_id),
                category_name: a.category_name,
                create_time: systime_to_i32(a.create_time),
                energy: 0, // TODO: 鍵能
                root_id: i64_to_id(a.root_id),
            })
            .collect())
    }
    fn field_party(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Party, juniper_from_schema::Walked>,
        party_name: String,
    ) -> Fallible<Party> {
        let party = party::get_party_by_name(&ex.context().get_pg_conn()?, &party_name)?;
        Ok(Party {
            id: i64_to_id(party.id),
            party_name: party.party_name,
            board_id: party.board_id.map(|id| i64_to_id(id)),
            chairman_id: party.chairman_id,
            energy: party.energy,
        })
    }
    fn field_my_party_list(
        &self,
        ex: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Party, juniper_from_schema::Walked>,
        board_name: Option<String>,
    ) -> Fallible<Vec<Party>> {
        let user_id = ex
            .context()
            .get_id()
            .ok_or(Error::new_logic("尚未登入", 401))?;
        // TODO 用 join?
        use db_schema::party_members;
        let conn = ex.context().get_pg_conn()?;
        let party_ids = party_members::table
            .filter(party_members::dsl::user_id.eq(user_id))
            .select(party_members::dsl::party_id)
            .load::<i64>(&conn)?;

        use db_schema::parties::dsl;
        let mut query = dsl::parties.into_boxed();
        if let Some(board_name) = board_name {
            let board = forum::get_board_by_name(&conn, &*board_name)?;
            query = query.filter(dsl::board_id.eq(board.id));
        }

        let party_vec = query
            .filter(dsl::id.eq_any(party_ids))
            .load::<db_models::Party>(&conn)?;
        Ok(party_vec
            .into_iter()
            .map(|p| Party {
                id: i64_to_id(p.id),
                party_name: p.party_name,
                board_id: p.board_id.map(|id| i64_to_id(id)),
                chairman_id: p.chairman_id,
                energy: p.energy,
            })
            .collect())
    }
    fn field_check_board_name_valid(
        &self,
        ex: &juniper::Executor<'_, Context>,
        name: String,
    ) -> Fallible<Option<String>> {
        match forum::check_board_name_valid(&ex.context().get_pg_conn()?, &name) {
            Ok(_) => Ok(None),
            Err(err) => Ok(Some(format!("{}", err))),
        }
    }
    fn field_check_party_name_valid(
        &self,
        ex: &juniper::Executor<'_, Context>,
        name: String,
    ) -> Fallible<Option<String>> {
        match party::check_party_name_valid(&ex.context().get_pg_conn()?, &name) {
            Ok(_) => Ok(None),
            Err(err) => Ok(Some(format!("{}", err))),
        }
    }
    fn field_check_article_content_valid(
        &self,
        ex: &juniper::Executor<'_, Context>,
        content: Vec<String>,
        board_name: String,
        category_name: String,
    ) -> Fallible<Vec<Option<String>>> {
        // TODO: 想辦法快取住分類
        let board = forum::get_board_by_name(&ex.context().get_pg_conn()?, &board_name)?;
        let category = forum::get_category(&ex.context().get_pg_conn()?, &category_name, board.id)?;
        let c_body = forum::CategoryBody::from_string(&category.body).unwrap();
        if c_body.structure.len() != content.len() {
            Err(Error::new_logic("結構長度有誤", 403))
        } else {
            Ok(content
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    c_body.structure[i]
                        .parse_content(c)
                        .err()
                        .map(|err| format!("{:?}", err)) // TODO catch all or catch LogicalError only?
                })
                .collect())
        }
    }
}

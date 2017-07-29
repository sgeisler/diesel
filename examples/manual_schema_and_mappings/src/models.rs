use super::schema::posts;
use diesel;
use diesel::backend::Backend;

use diesel::insertable::{ColumnInsertValue, Insertable, InsertValues};
use diesel::expression::helper_types::AsExpr;
use diesel::query_builder::insert_statement::{IntoInsertStatement, InsertStatement};
use diesel::expression::{AsExpression, Expression};

pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl <DB, ST> diesel::Queryable<ST, DB> for Post where
    DB: diesel::backend::Backend + diesel::types::HasSqlType<ST>,
    (i32, String, String, bool): diesel::types::FromSqlRow<ST, DB> {
    type Row = (i32, String, String, bool);
    fn build((id, title, body, published): Self::Row) -> Self {
        Post{id: id,
            title: title,
            body: body,
            published: published,}
    }
}

pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


impl <'a, 'insert, DB> Insertable<posts::table, DB> for &'insert NewPost<'a>
where DB: Backend,
(
    ColumnInsertValue<posts::title, AsExpr<&'insert &'a str, posts::title>>,
    ColumnInsertValue<posts::body, AsExpr<&'insert &'a str, posts::body>>
): InsertValues<DB>
{
    type Values = (
        ColumnInsertValue<posts::title, AsExpr<&'insert &'a str, posts::title>>,
        ColumnInsertValue<posts::body, AsExpr<&'insert &'a str, posts::body>>
    );
    #[allow(non_shorthand_field_patterns)]
    fn values(self) -> Self::Values {
        let NewPost { title: ref title, body: ref body } = *self;
        (
            ColumnInsertValue::Expression(
                posts::title,
                AsExpression::<<posts::title as Expression>::SqlType>::as_expression(title)
            ),
            ColumnInsertValue::Expression(
                posts::body,
                AsExpression::<<posts::body as Expression>::SqlType>::as_expression(body)
            )
        )
     }
}

impl<'a: 'insert, 'insert, Op> IntoInsertStatement<posts::table, Op> for &'insert NewPost<'a> {
    type InsertStatement = InsertStatement<posts::table, Self, Op>;
    fn into_insert_statement(self, target: posts::table, operator: Op) -> Self::InsertStatement {
        InsertStatement::no_returning_clause(target, self, operator)
    }
}

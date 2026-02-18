use std::str::FromStr;

use sea_orm::{Order, prelude::Expr, sea_query::SimpleExpr};

pub const TAKE_DEF: u64 = 20;
pub const TAKE_MAX: u64 = 100;
pub const SKIP_DEF: u64 = 0;

pub fn skip(skip: Option<u64>, take: Option<u64>) -> (u64, u64) {
    let skip = skip.unwrap_or(SKIP_DEF);
    let mut take = take.unwrap_or(TAKE_DEF);

    if take > TAKE_MAX {
        take = TAKE_MAX;
    }

    (skip, take)
}

pub fn order(desc: Option<bool>, default: bool) -> Order {
    let is_desc = desc.unwrap_or(default);
    if is_desc { Order::Desc } else { Order::Asc }
}

pub fn column<C>(column: Option<String>, default: C) -> C
where
    C: FromStr,
{
    column.and_then(|s| s.parse::<C>().ok()).unwrap_or(default)
}

pub fn column_cust(allowed: Vec<&str>, col: Option<String>, default: &str) -> SimpleExpr {
    let def = default.to_string();
    let col = match col {
        Some(col) => {
            if !allowed.contains(&col.as_str()) {
                def
            } else {
                col
            }
        }
        None => def,
    };

    Expr::cust(col)
}

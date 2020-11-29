use std::fmt::*;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;

use crate::error::TourneyError;

#[derive(PartialEq, Debug, Eq, AsExpression)]
#[sql_type = "Integer"]
pub enum GameLevel {
    Qualification,
    Quarterfinal,
    Semifinal,
    Final,
}

impl<DB: Backend> ToSql<Integer, DB> for GameLevel
    where
        i32: ToSql<Integer, DB>,
    {
        fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        let v = match *self {
            GameLevel::Qualification => 0,
            GameLevel::Quarterfinal => 1,
            GameLevel::Semifinal => 2,
            GameLevel::Final => 3,
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Integer, DB> for GameLevel
    where
        i32: FromSql<Integer, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
            let v = i32::from_sql(bytes)?;
            match v {
                0 => Ok(GameLevel::Qualification),
                1 => Ok(GameLevel::Quarterfinal),
                2 => Ok(GameLevel::Semifinal),
                3 => Ok(GameLevel::Final),
                x => Err(TourneyError::InternalServerError(format!("Unrecognized game level type `{}` while deserializing GameLevel", x)).to_string().into())
            }
    }
}

impl Into<GameLevel> for i32 {
    fn into(self) -> GameLevel {
        match self {
            0 => GameLevel::Qualification,
            1 => GameLevel::Quarterfinal,
            2 => GameLevel::Semifinal,
            _ => GameLevel::Final // Into doesn't allow errors, but this will never not be `1` because of the `ToSql` impl
        }
    }
}

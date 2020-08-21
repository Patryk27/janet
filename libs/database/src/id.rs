use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{Sqlite, SqliteRow};
use sqlx::{Database, Error, Row};
use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Id<T> {
    id: Uuid,
    _model: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _model: Default::default(),
        }
    }
}

impl<T> Copy for Id<T> {
    //
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            _model: Default::default(),
        }
    }
}

impl<T> PartialEq<Id<T>> for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {
    //
}

impl<T> PartialOrd<Id<T>> for Id<T> {
    fn partial_cmp(&self, other: &Id<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T> sqlx::Type<Sqlite> for Id<T> {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        String::type_info()
    }
}

impl<'q, T> sqlx::Encode<'q, Sqlite> for Id<T> {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        self.id.to_string().encode_by_ref(buf)
    }
}

impl<'r, T> sqlx::Decode<'r, Sqlite> for Id<T> {
    fn decode(value: <Sqlite as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let id = String::decode(value)?;
        let id = Uuid::from_str(&id)?;

        Ok(Self {
            id,
            _model: Default::default(),
        })
    }
}

impl<'r, T> sqlx::FromRow<'r, SqliteRow> for Id<T> {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        Ok(row.try_get(0)?)
    }
}

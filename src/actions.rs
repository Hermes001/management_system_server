use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

pub fn find_user_by_id(
    uid: &str,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let receive = user
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(receive)
}

pub fn insert_new_user(
    np: &str,
    pd: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let receive = models::User {
        id: Uuid::new_v4().to_string(),
        user_name: np.to_owned(),
        password: pd.to_owned(),
    };

    diesel::insert_into(user).values(&receive).execute(conn);

    Ok(receive)
}

pub fn check_user(
    name: &str,
    pwd: &str,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;
    let receive = user
        .filter(user_name.eq(name))
        .filter(password.eq(pwd))
        .first::<models::User>(conn)
        .optional()?;

    Ok(receive)
}

pub fn query_stock(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Commodity>>, diesel::result::Error> {
    use crate::schema::commodity::dsl::*;
    let receive = commodity
        .filter(quantity.gt(0))
        .load::<models::Commodity>(conn)
        .optional()?;
    Ok(receive)
}

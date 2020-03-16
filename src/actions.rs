use diesel::prelude::*;
use uuid::Uuid;

use crate::models;

<<<<<<< HEAD
// pub fn find_user_by_id(
//     uid: Uuid,
//     conn: &PgConnection,
// ) -> Result<Option<models::User>, diesel::result::Error> {
//     use crate::schema::users::dsl::*;

//     let user = users
//         .filter(id.eq(uid.to_string()))
//         .first::<models::User>(conn)
//         .optional()?;

//     Ok(user)
// }

pub fn insert_new_user(
    np: &str,
    pd: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
=======
pub fn find_user_by_id(uid: Uuid, conn: &PgConnection) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(np: &str, pd: &str, conn: &PgConnection) -> Result<models::User, diesel::result::Error> {
>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        user_name: np.to_owned(),
        password: pd.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
<<<<<<< HEAD
}
=======
}
>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75

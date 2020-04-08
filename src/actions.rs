use diesel::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models;

// 通过id查找用户
pub fn find_user_by_id(
    conn: &PgConnection,
    uid: &str,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;

    let receive = user
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(receive)
}

// 添加用户
pub fn add_user(
    conn: &PgConnection,
    np: &str,
    pd: &str,
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

// 检查用户
pub fn check_user(
    conn: &PgConnection,
    name: &str,
    pwd: &str,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::user::dsl::*;
    let receive = user
        .filter(user_name.eq(name))
        .filter(password.eq(pwd))
        .first::<models::User>(conn)
        .optional()?;

    Ok(receive)
}

// 查询库存
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

// 添加商品
pub fn add_commodity(
    conn: &PgConnection,
    map: HashMap<String, String>,
) -> Result<models::Commodity, diesel::result::Error> {
    use crate::schema::commodity::dsl::*;
    let receive = models::Commodity {
        id: Uuid::new_v4().to_string(),
        assortment_id: map["assortment_id"].to_owned(),
        model: Some(map["model"].to_owned()),
        description: Some(map["description"].to_owned()),
        price_retrieve: map["price_retrieve"].parse().unwrap(),
        price_sale: Some(map["price_sale"].parse().unwrap()),
        user_id: map["user_id"].to_owned(),
        memory: Some(map["memory"].parse().unwrap()),
        harddisk: Some(map["harddisk"].parse().unwrap()),
        harddisk_type_id: Some(map["harddisk_type_id"].to_owned()),
        gpu: Some(map["gpu"].to_owned()),
        cpu: Some(map["cpu"].to_owned()),
        price_retrieve_external: Some(map["price_retrieve_external"].parse().unwrap()),
        price_sale_external: Some(map["price_sale_external"].parse().unwrap()),
        photo_url: Some(map["photo_url"].to_owned()),
        quantity: map["quantity"].parse().unwrap(),
    };
    diesel::insert_into(commodity)
        .values(&receive)
        .execute(conn);
    Ok(receive)
}

// 添加品类
pub fn add_assortment(
    conn: &PgConnection,
    map: HashMap<String, String>,
) -> Result<models::Assortment, diesel::result::Error> {
    use crate::schema::assortment::dsl::*;
    let receive = models::Assortment {
        id: Uuid::new_v4().to_string(),
        name: map["name"].to_owned(),
    };
    diesel::insert_into(assortment)
        .values(&receive)
        .execute(conn);
    Ok(receive)
}

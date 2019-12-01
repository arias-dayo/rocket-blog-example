use crate::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct Posts {
    pub title: String,
    pub text: String,
}


#[derive(Queryable)]
pub struct Elections {
    pub id: i32,
    pub title: String,
    pub published: bool,
}

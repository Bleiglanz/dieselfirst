use super::schema::things;

#[derive(Insertable)]
#[table_name="things"]
pub struct NewThing<'a> {
    pub name: &'a str,
    pub description: &'a str,
}


#[derive(Queryable)]
pub struct Thing {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub sometext: String,
    pub item: i32,
}

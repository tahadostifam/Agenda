use diesel::prelude::*;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use crate::schema::todos;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub subject: String,
    pub context: String
}

impl Serialize for Todo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    
    where 
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Todo", 3)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("subject", &self.subject)?;
        s.serialize_field("context", &self.context)?;
        s.end()
    }
}


#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub subject: &'a str,
    pub context: &'a str
}
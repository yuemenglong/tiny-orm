use mysql::Value;
use mysql::Row;

#[derive(Debug)]
pub struct FieldMeta {
    pub name: String,
    pub ty: String,
    pub raw_ty: String,
    pub length: u32,
    pub nullable: bool,
}

macro_rules! value_from {
    ($SELF:ident,$FIELD:ident,$TYPE:ty) => (Value::from(&$SELF.$FIELD));
}

fn db_type_string(meta: &FieldMeta) -> String {
    let db_type = match meta.ty.as_ref() {
            "i32" => "INTEGER",
            "String" => "VARCHAR(128)",
            "Date" => "DATE",
            "DateTime" => "DATETIME",
            _ => unreachable!(),
        }
        .to_string();
    let nullable = match meta.nullable {
        true => "",
        false => " NOT NULL",
    };
    format!("{}{}", db_type, nullable)
}

pub trait Entity {
    fn set_id(&mut self, id: u64);
    fn get_id(&self) -> Option<u64>;
    fn get_name() -> String;
    fn get_field_meta() -> Vec<FieldMeta>;
    fn get_params(&self) -> Vec<(String, Value)>;
    fn from_row(mut row: Row) -> Self;

    fn get_create_table() -> String {
        let mut vec = vec!["`id` BIGINT PRIMARY KEY AUTO_INCREMENT".to_string()];
        let table = Self::get_name();
        for meta in Self::get_field_meta() {
            vec.push(format!("`{}` {}", meta.name, db_type_string(&meta)));
        }
        format!("CREATE TABLE IF NOT EXISTS `{}` ({})",
                table,
                vec.join(", "))
    }
    fn get_drop_table() -> String {
        format!("DROP TABLE IF EXISTS `{}`", Self::get_name())
    }
    
    fn get_field_list() -> String {
        let mut vec = Vec::new();
        vec.push("`id`".to_string());
        for meta in Self::get_field_meta() {
            vec.push(format!("`{}`", meta.name));
        }
        vec.join(", ")
    }
    fn get_prepare() -> String {
        let mut vec = Vec::new();
        for meta in Self::get_field_meta() {
            vec.push(format!("`{}` = :{}", meta.name, meta.name));
        }
        vec.join(", ")
    }
    fn get_params_id(&self) -> Vec<(String, Value)> {
        vec![("id".to_string(), Value::from(self.get_id()))]
    }
}

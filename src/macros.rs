#[macro_export]
macro_rules! cond {
    ($($FIELD:ident$OP:tt$E:expr),*)=>{{
        let mut vec = Vec::new();
        $(vec.push(cond_item!($FIELD$OP$E));)*
        vec
    }}
}

macro_rules! cond_item {
    ($FIELD:ident=$E:expr) => {{
        Cond::Eq(stringify!($FIELD).to_string(), Value::from($E))
    }};
    ($FIELD:ident>$E:expr) => {{
        Cond::Gt(stringify!($FIELD).to_string(), Value::from($E))
    }};
}

macro_rules! row_take {
    ($FIELD:ident, Option<$TYPE:ty>, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD));
        ret
    }};
    ($FIELD:ident, $TYPE:ty, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD));
        ret.unwrap()
    }};
}

#[macro_export]
macro_rules! entity {
    (struct $ENTITY:ident{
        $($FIELD:ident:$TYPE:ty,)*
    })=>{
        #[derive(Debug, Clone)]
        struct $ENTITY{
            id: Option<u64>,
            $($FIELD:$TYPE,)*
        }

        impl Entity for $ENTITY{
            fn get_table()->String{
                format!("`{}`", stringify!($ENTITY))
            }
            fn set_id(&mut self, id:u64){
                self.id = Some(id);
            }
            fn get_id_cond(&self)->String{
                format!("`id` = {}", self.id.unwrap())
            }
            fn get_fields()->String{
                let mut vec = Vec::new();
                vec.push("`id`".to_string());
                $(vec.push(format!("`{}`", stringify!($FIELD)));)*
                vec.join(", ")
            }
            fn get_prepare()->String{
                let mut vec = Vec::new();
                $(vec.push(format!("`{}` = :{}", stringify!($FIELD), stringify!($FIELD)));)*
                vec.join(", ")
            }
            fn get_params(&self)->Vec<(String, Value)>{
                let mut vec = Vec::new();
                $(vec.push((stringify!($FIELD).to_string(), Value::from(&self.$FIELD)));)*
                vec
            }
            fn get_params_id(&self)->Vec<(String, Value)>{
                vec![("id".to_string(), Value::from(self.id))]
            }
            fn from_row(mut row: mysql::conn::Row)->$ENTITY{
                $ENTITY{
                    id: row_take!(id, Option<u64>, row),
                    $($FIELD: row_take!($FIELD, $TYPE, row),)*
                }
            }
        }
    }
}
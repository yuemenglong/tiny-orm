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

#[macro_export]
macro_rules! datetime {
    ($Y:expr,$M:expr,$D:expr) => (datetime!($Y,$M,$D,0,0,0,0));
    ($Y:expr,$M:expr,$D:expr,$H:expr,$m:expr,$S:expr) => (datetime!($Y,$M,$D,$H,$m,$S,0));
    ($Y:expr,$M:expr,$D:expr,$H:expr,$m:expr,$S:expr,$NS:expr) => {{
        let date = Date::from_ymd($Y,$M,$D);
        let time = Time::from_hms_milli($H, $m, $S, $NS);
        DateTime::new(date, time)
    }};
}

#[macro_export]
macro_rules! row_take {
    ($FIELD:ident, Option<$TYPE:ty>, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD));
        ret
    }};
    ($FIELD:ident, $TYPE:ty, $ROW:ident) => {{
        let ret = $ROW.take(stringify!($FIELD)).unwrap();
        ret
    }};
}

#[macro_export]
macro_rules! value_from {
    ($SELF:ident,$FIELD:ident,$TYPE:ty) => (Value::from(&$SELF.$FIELD));
}

#[macro_export]
macro_rules! entity_type {
    ($TYPE:ty)=>{{
        match stringify!($TYPE){
            "i32"=>"INTEGER",
            "String"=>"VARCHAR(128)",
            "Date"=>"DATE",
            "DateTime"=>"DATETIME",
            _=>unreachable!(),
        }
    }}
}

#[macro_export]
macro_rules! entity_type_nullable {
    (Option<$TYPE:ty>)=>(true);
    ($TYPE:ty)=>(false);
}

#[macro_export]
macro_rules! entity_ty {
    (Option<$TYPE:ty>)=>(entity_ty!($TYPE));
    ($TYPE:ty)=>(stringify!($TYPE));
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
            fn set_id(&mut self, id:u64){
                self.id = Some(id);
            }
            fn get_id(&self)->Option<u64>{
                self.id
            }
            fn get_name()->String{
                stringify!($ENTITY).to_string()
            }
            fn get_field_meta()->Vec<FieldMeta>{
                let mut vec = Vec::new();
                $(vec.push(FieldMeta{
                    name: stringify!($FIELD).to_string(),
                    ty:entity_ty!($TYPE).to_string(), 
                    raw_ty: stringify!($TYPE).to_string(),
                    length: 0,
                    nullable: entity_type_nullable!($TYPE),
                });)*
                vec
            }
            fn get_params(&self)->Vec<(String, Value)>{
                let mut vec = Vec::new();
                $(vec.push((stringify!($FIELD).to_string(), value_from!(self,$FIELD,$TYPE)));)*
                vec
            }
            fn from_row(mut row: Row)->$ENTITY{
                $ENTITY{
                    id: row_take!(id, Option<u64>, row),
                    $($FIELD: row_take!($FIELD, $TYPE, row),)*
                }
            }
        }
    }
}

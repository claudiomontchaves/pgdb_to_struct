use app_properties::AppProperties;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::fs;

pub struct Field {
    name: String,
    data_type: String,
}

#[tokio::main]
async fn main() {
    let app_props = AppProperties::new();
    let db_pool = init_postgres_pool(app_props.clone()).await;

    let schema = app_props.get("db_schema");
    let tables = parse_table_name(app_props.get("tables"));
    let mut use_serde = false;
    if app_props.get("use_serde") == "true" {
        use_serde = true;
    }

    prepare_gen_folder();

    for table in tables {
        println!(">> Creating struct for table: {}", table);
        let fields = read_table_fields(schema, table.as_str(), &db_pool).await;
        let struct_content = gen_struct(table.clone(), fields, use_serde);
        write_struct_file(table, struct_content);
    }
}

pub async fn read_table_fields(schema: &str, table_name: &str, db_pool: &PgPool) -> Vec<Field> {
    let sql = format!("SELECT column_name, data_type FROM information_schema.columns WHERE table_schema = '{}' AND table_name = '{}'", schema, table_name);
    let rows = sqlx::query(sql.as_str()).fetch_all(db_pool).await.unwrap();

    let mut fields: Vec<Field> = Vec::new();

    for row in rows {
        fields.push(Field {
            name: row.try_get::<String, _>("column_name").unwrap_or_default(),
            data_type: row.try_get::<String, _>("data_type").unwrap_or_default(),
        });
    }
    fields
}

async fn init_postgres_pool(app_props: AppProperties) -> PgPool {
    let db_uri = format!(
        "postgres://{}:{}@{}:{}/{}",
        app_props.get("db_user"),
        app_props.get("db_password"),
        app_props.get("db_host"),
        app_props.get("db_port"),
        app_props.get("db_name")
    );

    PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_uri)
        .await
        .expect("Unable to connect to Postgres")
}

pub fn parse_table_name(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|table_name| table_name.trim().to_string())
        .collect()
}

pub fn gen_struct(table_name: String, fields: Vec<Field>, use_serde: bool) -> String {
    let mut content = "".to_string();

    if use_serde {
        content.push_str("use serde::{Deserialize, Serialize};\n\n");
        content.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    } else {
        content.push_str("#[derive(Debug)]\n");
    }

    content.push_str("pub struct ");
    content.push_str(snake_to_camel(table_name.as_str()).as_str());
    content.push_str(" {\n");

    for field in fields {
        let rust_type = to_rust_type(field.data_type.as_str());
        if use_serde && rust_type == "String" {
            content.push_str("    #[serde(skip_serializing_if = \"String::is_empty\")]\n");
        }
        content.push_str("    pub ");
        content.push_str(field.name.as_str());
        content.push_str(": ");
        content.push_str(rust_type);
        content.push_str(",\n");
    }
    content.push_str("}\n");
    content
}

fn write_struct_file(table_name: String, content: String) {
    let file_name = format!("gen/{}.rs", table_name);
    fs::write(file_name, content).expect("Unable to write file");
}

fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn to_rust_type(pg_type: &str) -> &str {
    match pg_type {
        "smallint" => "i16",
        "int2" => "i16",
        "integer" => "i32",
        "int" => "i32",
        "int4" => "i32",
        "bigint" => "i64",
        "int8" => "i64",
        pg_type if pg_type.starts_with("numeric") => "bigdecimal::BigDecimal",
        pg_type if pg_type.starts_with("decimal") => "bigdecimal::BigDecimal",
        "real" => "f32",
        "float4" => "f32",
        "double precision" => "f64",
        "float8" => "f64",
        "smallserial" => "i16",
        "serial2" => "i16",
        "serial" => "i32",
        "serial4" => "i32",
        "bigserial" => "i64",
        "serial8" => "i64",
        pg_type if pg_type.starts_with("char") => "String",
        pg_type if pg_type.starts_with("varchar") => "String",
        "text" => "String",
        pg_type if pg_type.starts_with("timestamp") => "chrono::DateTime",
        "date" => "chrono::NaiveDate",
        pg_type if pg_type.starts_with("time") => "chrono::NaiveTime",
        "bool" => "bool",
        "boolean" => "bool",
        "uuid" => "uuid::Uuid",
        _ => "String",
    }
}

fn prepare_gen_folder() {
    let folder_name = "gen";
    if let Ok(metadata) = fs::metadata(folder_name) {
        if metadata.is_dir() {
            fs::remove_dir_all(folder_name).unwrap();
        }
    }
    fs::create_dir(folder_name).unwrap();
}

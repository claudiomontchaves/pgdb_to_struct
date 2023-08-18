use super::*;
use std::path::Path;

#[test]
fn to_rust_type_test() {
    assert_eq!(to_rust_type("smallint"), "i16");
    assert_eq!(to_rust_type("int2"), "i16");
    assert_eq!(to_rust_type("integer"), "i32");
    assert_eq!(to_rust_type("int"), "i32");
    assert_eq!(to_rust_type("int4"), "i32");
    assert_eq!(to_rust_type("bigint"), "i64");
    assert_eq!(to_rust_type("numeric"), "bigdecimal::BigDecimal");
    assert_eq!(to_rust_type("decimal"), "bigdecimal::BigDecimal");
    assert_eq!(to_rust_type("real"), "f32");
    assert_eq!(to_rust_type("float4"), "f32");
    assert_eq!(to_rust_type("double precision"), "f64");
    assert_eq!(to_rust_type("float8"), "f64");
    assert_eq!(to_rust_type("smallserial"), "i16");
    assert_eq!(to_rust_type("serial2"), "i16");
    assert_eq!(to_rust_type("serial"), "i32");
    assert_eq!(to_rust_type("serial4"), "i32");
    assert_eq!(to_rust_type("bigserial"), "i64");
    assert_eq!(to_rust_type("serial8"), "i64");
    assert_eq!(to_rust_type("char"), "String");
    assert_eq!(to_rust_type("varchar"), "String");
    assert_eq!(to_rust_type("text"), "String");
    assert_eq!(to_rust_type("timestamp"), "chrono::DateTime");
    assert_eq!(to_rust_type("date"), "chrono::NaiveDate");
    assert_eq!(to_rust_type("time"), "chrono::NaiveTime");
    assert_eq!(to_rust_type("bool"), "bool");
    assert_eq!(to_rust_type("boolean"), "bool");
    assert_eq!(to_rust_type("uuid"), "uuid::Uuid");
    assert_eq!(to_rust_type("unknown"), "String");
}

#[test]
fn create_delete_gen_folder_test() {
    prepare_gen_folder();
    assert!(Path::new("gen").exists());
    fs::remove_dir_all("gen").unwrap();
    assert!(!Path::new("gen").exists());
}

#[test]
fn snake_to_camel_test() {
    assert_eq!(snake_to_camel("first_name"), String::from("FirstName"));
    assert_eq!(
        snake_to_camel("first_and_second_name"),
        String::from("FirstAndSecondName")
    );
}

#[test]
fn parse_table_name_test() {
    let tables = parse_table_name("customer, order, order_item");
    assert_eq!(tables.len(), 3);
    assert_eq!(tables[0], "customer");
    assert_eq!(tables[1], "order");
    assert_eq!(tables[2], "order_item");

    let tables_2 = parse_table_name("customer");
    assert_eq!(tables_2.len(), 1);
    assert_eq!(tables_2[0], "customer");
}

#[test]
fn gen_struct_test() {
    let struct_content = gen_struct("student".to_string(), create_test_field_vector(), false);
    assert_eq!(prepare_expected_struct_content(), struct_content);
}

#[test]
fn gen_struct_with_serde_test() {
    let struct_content = gen_struct("student".to_string(), create_test_field_vector(), true);
    assert_eq!(prepare_expected_struct_content_with_serde(), struct_content);
}

fn create_test_field_vector() -> Vec<Field> {
    let field1 = Field {
        name: String::from("id"),
        data_type: String::from("uuid"),
    };

    let field2 = Field {
        name: String::from("name"),
        data_type: String::from("varchar"),
    };

    let field3 = Field {
        name: String::from("birthday"),
        data_type: String::from("date"),
    };

    let field4 = Field {
        name: String::from("grade"),
        data_type: String::from("numeric"),
    };

    vec![field1, field2, field3, field4]
}

fn prepare_expected_struct_content() -> String {
    r#"#[derive(Debug)]
pub struct Student {
    pub id: uuid::Uuid,
    pub name: String,
    pub birthday: chrono::NaiveDate,
    pub grade: bigdecimal::BigDecimal,
}
"#
    .to_string()
}

fn prepare_expected_struct_content_with_serde() -> String {
    r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub birthday: chrono::NaiveDate,
    pub grade: bigdecimal::BigDecimal,
}
"#
    .to_string()
}

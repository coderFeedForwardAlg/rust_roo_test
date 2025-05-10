use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io;
use convert_case::{Case, Casing};
use serde::Deserialize;
use sqlx::FromRow;
use std::io::Write;

#[derive(Debug)]
struct Col {
    name: String,
    col_type: String,
}

#[derive(Debug)]
struct Row {
    name: String,
    cols: Vec<Col>,
}

// Helper function to insert multiple PostgreSQL types for a single Rust type
fn insert_multiple(map: &mut HashMap<String, String>, rust_type: &str, postgres_types: &[&str]) {
    for pg_type in postgres_types {
        map.insert(pg_type.to_string(), rust_type.to_string());
    }
}

fn create_type_map() -> HashMap<String, String> {
    let mut type_map = HashMap::new();

    insert_multiple(&mut type_map, "bool", &["BOOL"]);
    insert_multiple(&mut type_map, "i8", &["CHAR"]);
    insert_multiple(&mut type_map, "i16", &["SMALLINT", "SMALLSERIAL", "INT2"]);
    insert_multiple(&mut type_map, "i32", &["INT", "SERIAL", "INT4", "INTEGER"]);
    insert_multiple(&mut type_map, "i64", &["BIGINT", "BIGSERIAL", "INT8"]);
    insert_multiple(&mut type_map, "f32", &["REAL", "FLOAT4"]);
    insert_multiple(&mut type_map, "f64", &["DOUBLE PRECISION", "FLOAT8"]);
    insert_multiple(&mut type_map, "&str", &["VARCHAR", "CHAR(N)", "TEXT", "NAME", "CITEXT"]);
    insert_multiple(&mut type_map, "Vec<u8>", &["BYTEA"]);
    insert_multiple(&mut type_map, "()", &["VOID"]);
    insert_multiple(&mut type_map, "PgInterval", &["INTERVAL"]);
    insert_multiple(&mut type_map, "PgMoney", &["MONEY"]);
    insert_multiple(&mut type_map, "PgLTree", &["LTREE"]);
    insert_multiple(&mut type_map, "PgLQuery", &["LQUERY"]);
    insert_multiple(&mut type_map, "PgCiText", &["CITEXT1"]);
    insert_multiple(&mut type_map, "PgCube", &["CUBE"]);
    insert_multiple(&mut type_map, "PgPoint", &["POINT"]);
    insert_multiple(&mut type_map, "PgLine", &["LINE"]);
    insert_multiple(&mut type_map, "PgLSeg", &["LSEG"]);
    insert_multiple(&mut type_map, "PgBox", &["BOX"]);
    insert_multiple(&mut type_map, "PgPath", &["PATH"]);
    insert_multiple(&mut type_map, "PgPolygon", &["POLYGON"]);
    insert_multiple(&mut type_map, "PgCircle", &["CIRCLE"]);
    insert_multiple(&mut type_map, "PgHstore", &["HSTORE"]);

    // Add the new pairs
    type_map.insert("NUMERIC".to_string(), "bigdecimal::Decimal".to_string());
    type_map.insert("TIMESTAMPTZ".to_string(), "chrono::DateTime<Utc>".to_string());
    type_map.insert("TIMESTAMP".to_string(), "chrono::NaiveDateTime".to_string());
    type_map.insert("DATE".to_string(), "chrono::NaiveDate".to_string());
    type_map.insert("TIME".to_string(), "chrono::NaiveTime".to_string());
    type_map.insert("TIMETZ".to_string(), "PgTimeTz".to_string());
    type_map.insert("UUID".to_string(), "uuid::Uuid".to_string());
    insert_multiple(&mut type_map, "ipnetwork::IpNetwork", &["INET", "CIDR"]);
    insert_multiple(&mut type_map, "std::net::IpAddr", &["INET", "CIDR"]);
    insert_multiple(&mut type_map, "ipnet::IpNet", &["INET", "CIDR"]);
    insert_multiple(&mut type_map, "mac_address::MacAddress", &["MACADDR"]);
    insert_multiple(&mut type_map, "bit_vec::BitVec", &["BIT", "VARBIT"]);
    insert_multiple(&mut type_map, "Json<T>", &["JSON", "JSONB"]); //  *******  TODO:fix ********* //
    insert_multiple(&mut type_map, "serde_json::Value", &["JSON", "JSONB"]);
    insert_multiple(&mut type_map, "&serde_json::value::RawValue", &["JSON", "JSONB"]);

    // Handle PgRange<T> types
    type_map.insert("INT8RANGE".to_string(), "PgRange<i64>".to_string());
    type_map.insert("INT4RANGE".to_string(), "PgRange<i32>".to_string());
    type_map.insert("TSRANGE".to_string(), "PgRange<PgTimestamp>".to_string()); // Assuming you have a PgTimestamp type
    type_map.insert("TSTZRANGE".to_string(), "PgRange<PgTimestampTz>".to_string()); // Assuming you have a PgTimestampTz type
    type_map.insert("DATERANGE".to_string(), "PgRange<PgDate>".to_string()); // Assuming you have a PgDate type
    type_map.insert("NUMRANGE".to_string(), "PgRange<PgNumeric>".to_string()); // Assuming you have a PgNumeric type


    type_map
}

fn extract_table_schemas(file_path: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let mut schemas = Vec::new();
    let lower_contents = contents.to_lowercase();
    let mut start_index = 0;

    while let Some(create_index) = lower_contents[start_index..].find("create table") {
        let start = start_index + create_index;
        if let Some(open_paren_index) = contents[start..].find('(') {
            let schema_start = start + open_paren_index + 1;
            if let Some(close_paren_index) = contents[schema_start..].find(");") {
                let schema_end = schema_start + close_paren_index;
                let schema = contents[schema_start..schema_end].trim().to_string();
                schemas.push(schema);
                start_index = schema_end + 2; // Move past ");"
            } else {
                break; // Handle potential errors if closing parenthesis isn't found
            }
        } else {
            break; // Handle potential errors if opening parenthesis isn't found
        }
        start_index = start + 1;
    }

    Ok(schemas)
}

// similar logic to extract_table_schemas, maybe combin funcitons
fn extract_clean_table_names(file_path: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let mut table_names = Vec::new();
    let lower_contents = contents.to_lowercase();
    let mut start_index = 0;

    while let Some(create_index) = lower_contents[start_index..].find("create table") {
        let start = start_index + create_index + "create table".len();
        // Find the start of the table name
        let name_start = contents[start..].trim_start();
        // Extract the table name
        let mut table_name = String::new();
        for c in name_start.chars() {
            if c == '(' || c == ' ' || c == '\n' || c == '\r' {
                break;
            }
            table_name.push(c);
        }

        let cleaned_name = table_name
            .split('.')
            .last()
            .unwrap_or(&table_name)
            .trim_matches('"')
            .to_string();
        table_names.push(cleaned_name);

        // Find the end of the current CREATE TABLE statement (look for ");")
        if let Some(end_index) = contents[start..].find(");") {
            start_index = start + end_index + 2; // Move past ");"
        } else {
            break; // Handle case where ");" is not found (malformed SQL)
        }
    }

    Ok(table_names)
}

fn extract_column_info(schema: &str) -> Vec<Col> {
    let column_definitions: Vec<&str> = schema.split(',').map(|s| s.trim()).collect();
    let mut columns_info = Vec::new();

    for definition in column_definitions {
        let parts: Vec<&str> = definition.split_whitespace().collect();
        if parts.len() >= 2 {
            let name = parts[0].to_string();
            let col_type = parts[1].to_string();
            columns_info.push(Col { name, col_type });
        } else if parts.len() == 1 {
            // Handle cases with only a name (e.g., constraints)
            let name = parts[0].to_string();
            columns_info.push(Col { name, col_type: "".to_string() });
        }
    }

    columns_info
}

fn process_sql_file(file_path: &str) -> Result<(), io::Error> {
    let table_names = extract_clean_table_names(file_path)?;
    let schemas = extract_table_schemas(file_path)?;
    let mut rows: Vec<Row> = Vec::new();

    if table_names.len() != schemas.len() {
        eprintln!("Warning: Number of table names and schemas do not match!");
    }

    

    Ok(())
}

fn generate_struct(row: &Row, file_path: &str) -> Result<(), std::io::Error> {
    let type_map = create_type_map();
    let struct_name = row.name.to_case(Case::Pascal); // Convert table name to PascalCase
    let mut struct_string = format!("#[derive(Debug, Deserialize, FromRow)]\nstruct {} {{\n", struct_name);

    for col in &row.cols {
        let field_name = col.name.to_case(Case::Camel); // Convert column name to camelCase
        let rust_type = type_map.get(&col.col_type)
            .map(|s| s.as_str())
            .unwrap_or("String"); // Default to String if type not found
        struct_string.push_str(&format!("    {}: {},\n", field_name, rust_type));
    }

    struct_string.push_str("}\n");

    // Write the struct to the file
    // fs::write(file_path, struct_string)?;

    let mut file = OpenOptions::new()
        .write(true) // Enable writing to the file.
        .append(true) // Set the append mode.  Crucially, this makes it append.
        .create(true) // Create the file if it doesn't exist.
        .open(file_path)?; // Open the file, returning a Result.

    // Write the data to the file.
    file.write_all(struct_string.as_bytes())?;
    Ok(())
}

fn create_rows_from_sql(file_path: &str) -> Result<Vec<Row>, io::Error> {
    let table_names = extract_clean_table_names(file_path)?;
    let schemas = extract_table_schemas(file_path)?;
    let mut rows: Vec<Row> = Vec::new();

    if table_names.len() != schemas.len() {
        eprintln!("Warning: Number of table names and schemas do not match!");
    }

    for (table_name, schema) in table_names.iter().zip(schemas.iter()) {
        let cols = extract_column_info(schema);
        let row = Row {
            name: table_name.to_string(), // Convert &String to String
            cols,
        };
        rows.push(row);
    }


    Ok(rows)
}



fn main() -> Result<(), io::Error> {
    let rows = create_rows_from_sql("test.sql")?;
    // println!("Table names: {:?}", rows.iter().map(|row| row.name.clone()).collect::<Vec<String>>());
    for row in rows {
        println!("Row: {:?} \n", row);
        generate_struct(&row, "src/generated_struct.rs")?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs;

    #[test]
    fn test_extract_table_schemas() -> Result<(), io::Error> {
        let sql_content = r#"
        CREATE TABLE public."user" (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            favorite_color VARCHAR(50),
            height NUMERIC,
            age INTEGER,
            job VARCHAR(100)
        );

        CREATE TABLE product_details (
            product_id INTEGER PRIMARY KEY,
            description TEXT,
            price DECIMAL(10, 2)
        );


        create table order_items (
            order_id INTEGER,
            item_id INTEGER,
            quantity INTEGER
        );
        "#;
        fs::write("test.sql", sql_content)?;

        let expected_schemas = vec![
            "id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            favorite_color VARCHAR(50),
            height NUMERIC,
            age INTEGER,
            job VARCHAR(100)",
            "product_id INTEGER PRIMARY KEY,
            description TEXT,
            price DECIMAL(10, 2)",
            "order_id INTEGER,
            item_id INTEGER,
            quantity INTEGER",
        ];

        let schemas = extract_table_schemas("test.sql")?;
        assert_eq!(schemas.len(), expected_schemas.len());
        for (i, schema) in schemas.iter().enumerate() {
            assert_eq!(schema.trim(), expected_schemas[i].trim());
        }

        Ok(())
    }
}

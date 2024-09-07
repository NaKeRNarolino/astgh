use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::ptr::null;
use serde_json::{json, Map, Value};

fn parse_object(object: Map<String, Value>, structs: &mut String, last_obj_title: String, unique_ident: i32) -> Result<Value, ()> {
    dbg!(&object);

    let type_to_type: HashMap<&str, &str> = HashMap::from([
        ("number", "i32"),
        ("string", "String"),
        ("array", "Vec<i32>"),
        ("boolean", "bool"),
        ("object", "Map<String, Value>")
    ]);


    let mut current_res = String::new();
    let mut current_props: Map<String, Value> = Map::new();

    if let Some(one_of) = object.get("oneOf").cloned() {
        println!("ONEOF");
        let title = object.get("title").unwrap_or(&json!(format!("{last_obj_title}{unique_ident}"))).clone();

        if !title.is_string() {
            panic!("'title' property should be of type String");
        }

        let title = title.as_str().unwrap().replace(' ', "");

        let description = object.get("description").unwrap_or(&json!("UNDOCUMENTED")).clone();

        if !description.is_string() {
            panic!("'description' property should be of type String");
        }

        let description = description.as_str().unwrap();


        current_res.push_str(&format!("/// {description} \n"));
        current_res.push_str("#[derive(Clone, Serialize, Deserialize, Debug)]\n");
        current_res.push_str(&format!("pub struct {last_obj_title}{title} {{\n"));

        if !one_of.is_array() {
            panic!("'oneOf' should be an array")
        }



        let schema: Map<String, Value> = one_of.as_array().cloned().unwrap().first().unwrap().clone().as_object().unwrap().clone();

        let v: Map<String, Value> = parse_object(schema, structs, title, unique_ident + 1)?.as_object().unwrap().clone();

        dbg!(&v);

        for (k, v) in v {
            let t = v.as_str().unwrap();

            current_res.push_str(&format!("   pub {k}: {t},\n"))
        }

        structs.push_str(&current_res);
        structs.push_str("}\n\n ");
    } else if let Some(properties) = object.get("properties").cloned() {
        if !properties.is_object() {
            panic!("'properties' should be an object")
        }

        let properties: Map<String, Value> = properties.as_object().unwrap().clone();

        for (k, v) in properties {
            let props = v.as_object().unwrap().clone();

            let title = props.get("title").expect("should have 'title' param").as_str().unwrap().to_string();

            let type_id = props.get("type").expect("No type specified").as_str().unwrap_or("boolean");

            let name = type_to_type[type_id];

            current_props.insert(k, json!(name));
        }

        return Ok(json!(current_props));
    } else if let Some(items) = object.get("items").cloned() {
        return Ok(json!(type_to_type["array"]))
    }

    Ok(json!(""))
}

fn parse_schemas(file: String) -> String {
    let mut structs: String = String::new();
    let mut json: Vec<Value> = serde_json::from_str(&file).unwrap();

    for schema_raw in json {
        let is_obj = schema_raw.is_object();

        if !is_obj {
            panic!("Expected Object");
        }

        let obj = schema_raw.as_object().unwrap().clone();

        for (schema_id, schema_value) in obj {
            let is_obj = schema_value.is_object();

            if !is_obj {
                continue;
            }

            let obj = schema_value.as_object().unwrap().clone();

            parse_object(obj, &mut structs, "".to_string(), 0);
        }

    }


    structs
}

fn main() -> Result<(), ()> {
    let file = fs::read_to_string("./short_schema.json").expect("File 'short_schema.json' is not found!");

    let structs: String = parse_schemas(file);

    let _ = fs::write("./res.rs", structs);

    Ok(())
}
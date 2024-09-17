use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::ptr::null;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Number, Value};

#[derive(Clone, Debug)]
struct TitleDesc {
    name: String,
    title: String,
    description: String,
}


#[derive(Clone, Debug)]
enum Element {
    Number(TitleDesc),
    String(TitleDesc),
    Boolean(TitleDesc),
    Object(TitleDesc, Vec<Element>),
    Array(TitleDesc, String)
}

#[derive(Clone, Debug)]
struct Schema {
    schema: Vec<Element>
}

#[derive(PartialEq)]
enum ValType {
    Number,
    String,
    Object,
    Boolean,
    Array
}

fn get_val_type(type_id: String) -> ValType {
    if type_id == "number" || type_id == "integer" {
        ValType::Number
    } else if type_id == "string" {
        ValType::String
    } else if type_id == "bool" {
        ValType::Boolean
    } else if type_id == "object" {
        ValType::Object
    } else if type_id == "array" {
        ValType::Array
    } else {
        ValType::String
    }
}

fn get_key(val: Value, key: &str) -> String {
    let q = val.clone().as_object().unwrap().clone();
    let r = q.clone().get(key).cloned();

    r.unwrap().as_str().unwrap().to_string()
}

fn get_obj_props(val: Value) -> Vec<Element> {
    let mut props: Vec<Element> = vec![];

    let raw_props = val.as_object().unwrap().get("properties").unwrap().as_object().unwrap();

    for (k, v) in raw_props {
        let type_id = v.as_object().unwrap().get("type").unwrap().as_str().unwrap();

        let val_type = get_val_type(type_id.to_string());

        let title = get_key(v.clone(), "title");
        let description = get_key(v.clone(), "description");

        props.push(
            match val_type {
                ValType::Number => {
                    Element::Number(
                        TitleDesc {
                            title, description, name: k.clone()
                        }
                    )
                }
                ValType::String => {
                    Element::String(
                        TitleDesc {
                            title, description, name: k.to_string()
                        }
                    )
                }
                ValType::Object => {
                    Element::Object(
                        TitleDesc {
                            title, description, name: k.to_string()
                        },
                        get_obj_props(v.clone())
                    )
                }
                ValType::Boolean => {
                    Element::Boolean(
                        TitleDesc {
                            title, description, name: k.to_string()
                        },
                    )
                }
                ValType::Array => {
                    let el_type = {
                        let items = v.as_object().unwrap().get("items").unwrap();
                        if items.clone().is_object() {
                            match get_val_type(get_key(items.clone(), "type")) {
                                ValType::Number => {
                                    "f64".to_string()
                                }
                                ValType::String => {
                                    "String".to_string()
                                }
                                ValType::Boolean => {
                                    "bool".to_string()
                                }
                                ValType::Object => {
                                    "Map<String, serde_json::Value>".to_string()
                                }
                                ValType::Array => {
                                    format!("Vec<IDK>")
                                }
                            }
                        } else {
                            match get_val_type(get_key(items.as_array().unwrap().first().unwrap().clone(), "type")) {
                                ValType::Number => {
                                    "f64".to_string()
                                }
                                ValType::String => {
                                    "String".to_string()
                                }
                                ValType::Boolean => {
                                    "bool".to_string()
                                }
                                ValType::Object => {
                                    "Map<String, serde_json::Value>".to_string()
                                }
                                ValType::Array => {
                                    format!("Vec<IDK>")
                                }
                            }
                        }
                    };

                    Element::Array(
                        TitleDesc {
                            title, description, name: k.to_string()
                        },
                        el_type
                    )
                }
            }
        )
    }

    props
}


fn parse(source: String) -> Schema {
    let mut parsed: Schema = Schema {
        schema: vec![],
    };

    let ser: Vec<Value> = serde_json::from_str(&source).unwrap();

    dbg!(ser.clone());

    for val in ser {
        if get_val_type(get_key(val.clone(), "type")) == ValType::Object {
            let title = get_key(val.clone(), "title");
            let description = get_key(val.clone(), "description");
            let title_desc = TitleDesc {
                name: title.replace(' ', ""),
                description: description.to_string(), title: title.clone()
            };

            let props = get_obj_props(val);

            parsed.schema.push(
              Element::Object(title_desc, props)
            );
        } else {
            println!("Ignoring");
        }
    }

    dbg!(parsed.clone());
    parsed
}

fn gen(src: Schema, id_next: String) -> String {
    let mut string = String::new();

    for el in src.schema {
        if let Element::Object(ntd, props) = el {
            let title = ntd.clone().title;
            let description = ntd.clone().description;
            let name = ntd.clone().name;

            string.push_str(&format!("/// {description}\n"));
            string.push_str("#[derive(Clone, Debug, Serialize, Deserialize)]\n");
            string.push_str(&format!("struct {name}{id_next} {{\n"));

            for element in props {
                string.push_str(&format!("    {}: {}", match element.clone() {
                    Element::Number(n) => {
                        n.name
                    }
                    Element::String(n) => {
                        n.name
                    }
                    Element::Boolean(n) => {
                        n.name
                    }
                    Element::Object(n, _) => {
                        n.name
                    }
                    Element::Array(n, _) => {
                        n.name
                    }
                }, match element {
                    Element::Number(_) => {
                        "f64".to_string()
                    }
                    Element::String(_) => {
                        "String".to_string()
                    }
                    Element::Boolean(_) => {
                        "bool".to_string()
                    }
                    Element::Object(_, _) => {
                        "Map<String, serde_json::Value>".to_string()
                    }
                    Element::Array(_, t) => {
                        format!("Vec<{t}>")
                    }
                }));
                string.push_str(",\n")
            }

            string.push_str("}\n\n")
        }
    }


    string
}

fn main() -> Result<(), ()> {
    let file = fs::read_to_string("./short_schema.json").expect("File 'short_schema.json' is not found!");

    let structs = parse(file);

    let _ = fs::write("./res.rs", gen(structs, String::from("BlockComponent")));


    Ok(())
}
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value, Number};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ExtArgs {
    pub options: Vec<Option>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Option {
    pub name: String,

    #[serde(rename = "type")]
    pub the_type: String,

    pub description: String,

    pub default: Value,
}


fn main() {
    // let json = serde_json::to_string_pretty(&x).unwrap();
    // println!("{}", json);

    let opt0 = Option {
        name: "lang".to_string(),
        the_type: "string".to_string(),
        description: "the language".to_string(),
        default: Value::String("en".to_string()),
    };

    let n = Number::from(42);
    println!("{:?}", n);

    let v_number = Value::Number(n);
    println!("{:?}", v_number);

    let opt1 = Option {
        name: "num".to_string(),
        the_type: "int".to_string(),
        description: "the number of things".to_string(),
        default: v_number,
    };

    let ext_args = ExtArgs {
        options: vec![opt0, opt1],
    };

    let json = serde_json::to_string_pretty(&ext_args).unwrap();
    println!("{}", json);


    println!("");
    println!("");
    println!("");

    let input_str = r#"{
		"options": [
			{
				"name": "lang",
				"type": "string",
				"description": "the language",
				"default": "en"
			},
			{
				"name": "num",
				"type": "int",
				"description": "the number of things",
				"default": 42
			}
		]
	}
    "#;

    // dynamic deser
    // let maybe_obj = serde_json::from_str::<Value>(input_str);
    
    let maybe_ext_args = serde_json::from_str::<ExtArgs>(input_str);
    
    match maybe_ext_args {
        Ok(obj) => {
            println!("{:?}", obj);

            for option in obj.options {
                println!("name: {:?}", option.name);
                println!("type: {:?}", option.the_type);
                println!("default: {:?}", option.default);

                if option.default.is_string() {
                    let s = option.default.as_str().unwrap();
                    println!("default as string: {:?}", s);
                } else if option.default.is_i64() {
                    let n = option.default.as_i64().unwrap();
                    println!("default as i64: {:?}", n);
                } else {
                    panic!("unexpected default type");
                }
                
                println!("");
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic_option() {
//     }
// }

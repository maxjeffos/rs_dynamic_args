use serde_derive::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ExtensionMetadata {
    name: String,
    command: String,
    version: String,
    help_description: String,
    options: Vec<Option>,
}

// struct Option<T> where T: ToString {
//     name: String,
//     shorthand: String,
//     // option_type: string,
//     description: String,
//     usage: String,
// }

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Option {
    StringOption {
        name: String,
        shorthand: String,
        usage: String,
    },
    StringArrayOption {
        name: String,
        shorthand: String,
        usage: String,
    },
    BoolOption {
        name: String,
        shorthand: String,
        usage: String,
    },
    IntOption {
        name: String,
        shorthand: String,
        usage: String,
    },
}


fn main() {
    let option_lang = Option::StringOption {
        name: "lang".to_string(),
        shorthand: "l".to_string(),
        usage: "the language".to_string(),
    };

    let option_port = Option::IntOption {
        name: "port".to_string(),
        shorthand: "p".to_string(),
        usage: "the port".to_string(),
    };

    let x = ExtensionMetadata {
        name: "sclix-woof".to_string(),
        command: "woof".to_string(),
        version: "0.1.0".to_string(),
        help_description: "do ".to_string(),
        options: vec![option_lang, option_port],
    };

    let json = serde_json::to_string_pretty(&x).unwrap();
    println!("{}", json);

    let input_str = r#"{
        "name": "sclix-woof",
        "command": "woof",
        "version": "0.1.0",
        "help_description": "patch ascii art",
        "options": [
            {
                "name": "lang",
                "shorthand": "l",
                "type": "string",
                "usage": "the language you want to show"
            }
        ]
    }
    "#;

    let input_str = r#"
    {
		"name": "sclix-woof",
		"command": "woof",
		"version": "0.1.0",
		"description": "patch ascii art",
		"options": [
			{
				"name": "lang",
				"shorthand": "l",
				"type": "string",
				"default": "en",
				"description": "the language you want to show"
			},
			{
				"name": "num",
				"shorthand": "n",
				"type": "int",
				"default": 42,
				"description": "the number of things"
			}
		]
	}
    "#;


    // let maybe_obj = serde_json::from_str::<ExtensionMetadata>(input_str);

    // match maybe_obj {
    //     Ok(obj) => {
    //         println!("{:?}", obj);
    //     },
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    // }


    // if let Ok(metadata) = maybe_obj {
    //     println!("{:#?}", metadata);
    // } else {
    //     println!("{}", input_str);
    //     // panic!("{:?}", Error::from_str(input_str));
    // }
    
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic_option() {
//         let input_str = "{";
//         let args = 
//     }
// }

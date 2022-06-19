use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value, Number};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ExtMeta {
    pub name: String,
    pub options: Vec<CommandOption>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CommandOption {
    pub name: String,

    #[serde(rename = "type")]
    pub the_type: CommandOptionType,

    // pub description: String,

    pub default: Value,

    // can I have deser logic that makes sure that this is only set if type is string?
    pub allowed_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum CommandOptionType {
    #[serde(rename = "string")]
    String,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "string-array")]
    StringArray,
}

// the simple thing that I have a solution for in Golang 
pub fn spelunk_simple() {

    let json_str = r#"{
        "name": "test",
		"options": [
			{
				"name": "thing1",
				"type": "string",
				"default": "foo",
                "allowed_values": [
                    "foo",
                    "bar"
                ]
			},
			{
				"name": "thing2",
				"type": "number",
				"default": 42
			}
		]
	}"#;

    let maybe_ext_args = serde_json::from_str::<ExtMeta>(json_str);
    match maybe_ext_args {
        Ok(obj) => {
            println!("deserialized ok");
            println!("name: {}", obj.name);

            println!("pbj: {:#?}", obj);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    println!("sfsdf");
}

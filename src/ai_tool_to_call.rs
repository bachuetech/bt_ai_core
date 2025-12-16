use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

///Tools Returned by AI Model that the application needs to call to return an answer to the AI model.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolToCall{
    function: FunctionToCall
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionToCall{
    name: String,
    arguments: HashMap<String,Value>,
}

impl ToolToCall{
    pub fn new(function_name: String, function_args: HashMap<String, Value>) -> Self{
        let ftc = FunctionToCall{
            name: function_name,
            arguments: function_args,
        };

        Self{
            function: ftc,
        }
    }

    pub fn get_function_name(&self) -> &String{
        &self.function.name
    }

    pub fn get_arguments(&self) -> HashMap<String,String>{
        //&self.function.arguments
        //ToDo: Need a more elegant solution.
        let mut output:HashMap<String,String> = HashMap::new();

        for (key, value) in &self.function.arguments {
            // Convert each `Value` to a `String` using `to_string()`
            output.insert(key.to_owned(), value.to_string());
        }
        output
    }
}



//**********/
//UNIT TEST
//*********/
#[cfg(test)]
mod tests_tool_to_call{
    use std::collections::HashMap;

    use bt_logger::{build_logger, LogLevel, LogTarget};
    use serde_json::Value;

    use crate::ai_tool_to_call::{FunctionToCall, ToolToCall};


#[test]
fn test_tool_to_call(){
    build_logger("BACHUETECH", "BT.AI_TOOL2CALL", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
    let mut arg: HashMap<String, Value> = HashMap::new();
    arg.insert("ARG1".to_owned(), Value::String("Value1".to_owned()));
    let ftc = FunctionToCall{
        name: "FunctName".to_owned(),
        arguments: arg,
    };
    let ttc = ToolToCall{
        function: ftc,
    };

    assert_eq!(ttc.get_function_name(),"FunctName");
    assert_eq!(ttc.get_arguments().get("ARG1").unwrap(),"\"Value1\"");//It is a JSON Value
    
}

#[test]
fn test_tool_to_call_new(){
    build_logger("BACHUETECH", "BT.AI_TOOL2CALL", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
    let mut arg: HashMap<String, Value> = HashMap::new();
    arg.insert("ARG1".to_owned(), Value::String("Value1".to_owned()));

    let ttc = ToolToCall::new("FunctName".to_owned(), arg);

    assert_eq!(ttc.get_function_name(),"FunctName");
    assert_eq!(ttc.get_arguments().get("ARG1").unwrap(),"\"Value1\"");//It is a JSON Value
    
}

}
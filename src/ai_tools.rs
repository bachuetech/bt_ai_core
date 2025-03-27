use std::collections::{HashMap, HashSet};

use bt_file_utils::get_file;
use bt_logger::log_warning;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ai_config::SupportedFunctions;

const TOOLS_JSON_DEF: &str = "defs/tools-def.json";
const TOOLS_JSON_DEF_ENV_VAR_NAME: &str = "BT_AITOOLS_DEFJSONFILE";

#[derive(Debug)]
pub struct AIToolManager{
    tools: Option<Tools>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tools{
    tools: Vec<Tool>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tool{
    #[serde(rename = "type")]
    type_: String,  // "function"
    function: Function,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Function{
    name: String,
    description: String,
    parameters: FunctionParameters,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FunctionParameters{
    #[serde(rename = "type")]
    type_: String,
    properties: HashMap<String,ToolParamProperty>,
    required: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolParamProperty{
    #[serde(rename = "type")]
    type_ : String,
    description: String
}

impl AIToolManager {
    pub fn new() -> Self {
        let tools_def: String;
        match get_file(TOOLS_JSON_DEF_ENV_VAR_NAME, TOOLS_JSON_DEF){
            Ok(j_file_conf) => tools_def = j_file_conf,
            Err(e) => {
                log_warning!("new","Error loding JSON tools configuration file. Using Empty tools as default. Error: {}",e.to_string()); 
                return Self{tools: None, } //tools_def = "".to_owned();
            },
        }

        match serde_json::from_str(&tools_def) {
            Ok(t) => {
                Self{ tools: Some(t),} //json_tools: tools_def, tool_count: num_tools}
            }
            Err(e) => {
                log_warning!("AIToolManager:new", "Error loading tools or No tools available: {}", e) ;
                Self{tools: None, }//json_tools: "".to_owned(), tool_count: 0 }
            }
        }
    }

    pub fn get_common_tools(&self, functions: SupportedFunctions) ->Option<Vec<Tool>>{
        match functions{
            SupportedFunctions::NONE => None,
            SupportedFunctions::ALL =>  Some(self.tools.clone().unwrap().tools),
            SupportedFunctions::Functions(func) => {
                let set2: HashSet<String> = func.into_iter().collect();
                Some(self.tools.clone().unwrap().tools.into_iter().filter(|item| set2.contains(&item.function.name)).collect()) } ,
        }
    }
}

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
mod tests_ai_tools{
    use std::collections::HashMap;

    use bt_logger::{build_logger, LogLevel, LogTarget};
    use serde_json::Value;
    use crate::ai_config::SupportedFunctions;
    use super::{AIToolManager, FunctionToCall, ToolToCall};

    #[test]
    fn test_ai_toolmgr_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let aitm = AIToolManager::new();
        assert_eq!(aitm.tools.unwrap().tools.len(),3); 
    }

    #[test]
    fn test_ai_toolmgr_common_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let aitm = AIToolManager::new();
        let sf = SupportedFunctions::Functions(vec!["do_math_expressions".to_string()]);
        assert_eq!(aitm.get_common_tools(sf).unwrap()[0].function.name,"do_math_expressions"); //do_math_expressions is common
    }

    #[test]
    fn test_ai_toolmgr_common_all(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let aitm = AIToolManager::new();
        let sf = SupportedFunctions::ALL;
        assert_eq!(aitm.get_common_tools(sf.clone()).unwrap()[0].function.name,"get_current_weather"); 
        assert_eq!(aitm.get_common_tools(sf).unwrap()[2].function.name,"do_math_expressions"); 
    }

    #[test]
    fn test_ai_toolmgr_common_none(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let aitm = AIToolManager::new();
        let sf = SupportedFunctions::NONE;
        assert!(aitm.get_common_tools(sf.clone()).is_none()); 
    }

    #[test]
    fn test_ai_toolmgr_nocommon(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let aitm = AIToolManager::new();
        let sf = SupportedFunctions::Functions(vec!["do_nothing".to_string()]);
        println!("{:?}",&aitm.get_common_tools(sf.clone()));
        assert_eq!(aitm.get_common_tools(sf).unwrap().len(),0); //Zero function in common
    }

    #[test]
    fn test_tool_to_call(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
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
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let mut arg: HashMap<String, Value> = HashMap::new();
        arg.insert("ARG1".to_owned(), Value::String("Value1".to_owned()));

        let ttc = ToolToCall::new("FunctName".to_owned(), arg);

        assert_eq!(ttc.get_function_name(),"FunctName");
        assert_eq!(ttc.get_arguments().get("ARG1").unwrap(),"\"Value1\"");//It is a JSON Value
        
    }

}
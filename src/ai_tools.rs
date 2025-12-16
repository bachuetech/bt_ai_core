use std::{collections::{HashMap, HashSet}, error::Error};

use bt_file_utils::get_file;
use bt_logger::log_warning;
use serde::{Deserialize, Serialize};

use crate::ai_config::{AIConfig, SupportedFunctions};

const TOOLS_JSON_DEF: &str = "defs/tools-def.json";
const TOOLS_JSON_DEF_ENV_VAR_NAME: &str = "BT_AITOOLS_DEFJSONFILE";

#[derive(Debug)]
pub struct AIToolManager{
    tools: Option<Tools>,
    ai_config: AIConfig,
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
    pub fn new(run_environment: &String) -> Result<Self, Box<dyn Error>>  {
        let tools_def: String;
        match get_file(TOOLS_JSON_DEF_ENV_VAR_NAME, TOOLS_JSON_DEF){
            Ok(j_file_conf) => tools_def = j_file_conf,
            Err(e) => {
                log_warning!("new","Error loding JSON tools configuration file. Using Empty tools as default. Error: {}",e.to_string()); 
                return Ok(Self{
                    tools: None,
                    ai_config: AIConfig::new(&run_environment)?, 
                }) //tools_def = "".to_owned();
            },
        }

        match serde_json::from_str(&tools_def) {
            Ok(t) => {
                Ok(Self{ 
                    tools: Some(t),
                    ai_config: AIConfig::new(&run_environment)?,
                }) //json_tools: tools_def, tool_count: num_tools}
            }
            Err(e) => {
                log_warning!("AIToolManager:new", "Error loading tools or No tools available: {}", e) ;
                Ok(Self{
                    tools: None, 
                    ai_config: AIConfig::new(&run_environment)?,
                }) //json_tools: "".to_owned(), tool_count: 0 }
            }
        }
    }

    pub fn get_tools(&self, platform_name: &String, model_id: &String) -> Option<Vec<Tool>> {
        if let Some(p) = self.ai_config.get_models(platform_name) {
            if let Some(tool_model) = p.get(model_id) {
                    self.get_common_tools(tool_model.tools.clone())
            } else {
                if model_id.to_lowercase() == "default" { //This is a Stop condition. meaning there is no default defined ^^^
                    None 
                } else {
                    self.get_tools(platform_name, &"default".to_owned())
                }
            }
        } else {
            None
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



//**********/
//UNIT TEST
//*********/
#[cfg(test)]
mod tests_ai_tools{
    use bt_logger::{build_logger, LogLevel, LogTarget};
    use crate::ai_config::SupportedFunctions;
    use super::AIToolManager;

    #[test]
    fn test_ai_get_tools_ok(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let t = aitm.get_tools(&"OLLAMALOCAL".to_owned(), &"llama3.1".to_owned());
        assert_eq!(t.unwrap().len(),3); 
    }

    #[test]
    fn test_ai_get_tools_list(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let t = aitm.get_tools(&"OLLAMALOCAL".to_owned(), &"tlist".to_owned());
        assert_eq!(t.clone().unwrap().len(),1); 
        assert_eq!(t.unwrap()[0].function.name,"do_basic_math");
    }

    #[test]
    fn test_ai_get_tools_none(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let t = aitm.get_tools(&"OLLAMALOCAL".to_owned(), &"guardian".to_owned());
        assert!(t.is_none()); 
    }

    #[test]
    fn test_ai_get_tools_fakemodel(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let t = aitm.get_tools(&"OLLAMALOCAL".to_owned(), &"FAKEMODEL:ver123".to_owned());
        assert!(t.is_none());  
    }

    #[test]
    fn test_ai_get_tools_invplatf(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let t = aitm.get_tools(&"INVALID".to_owned(), &"llama3.1".to_owned());
        assert!(t.is_none());  
    }

    #[test]
    fn test_ai_get_tools_unknowenv(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"UNKNOWN".to_owned()).unwrap();
        let t = aitm.get_tools(&"OLLAMALOCAL".to_owned(), &"llama3.1".to_owned());
        assert!(t.is_none()); 
    }

    #[test]
    fn test_ai_toolmgr_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        assert_eq!(aitm.tools.unwrap().tools.len(),3); 
    }

    #[test]
    fn test_ai_toolmgr_common_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let sf = SupportedFunctions::Functions(vec!["do_math_expressions".to_string()]);
        assert_eq!(aitm.get_common_tools(sf).unwrap()[0].function.name,"do_math_expressions"); //do_math_expressions is common
    }

    #[test]
    fn test_ai_toolmgr_common_all(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let sf = SupportedFunctions::ALL;
        assert_eq!(aitm.get_common_tools(sf.clone()).unwrap()[0].function.name,"get_current_weather"); 
        assert_eq!(aitm.get_common_tools(sf).unwrap()[2].function.name,"do_math_expressions"); 
    }

    #[test]
    fn test_ai_toolmgr_common_none(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let sf = SupportedFunctions::NONE;
        assert!(aitm.get_common_tools(sf.clone()).is_none()); 
    }

    #[test]
    fn test_ai_toolmgr_nocommon(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );
        let aitm = AIToolManager::new(&"dev".to_owned()).unwrap();
        let sf = SupportedFunctions::Functions(vec!["do_nothing".to_string()]);
        println!("{:?}",&aitm.get_common_tools(sf.clone()));
        assert_eq!(aitm.get_common_tools(sf).unwrap().len(),0); //Zero function in common
    }



}
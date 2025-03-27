use std::{collections::HashMap, process};

use bt_app_codes::{labels::{AI_PLATFORM_LABEL, HOST_LABEL, PORT_LABEL, SERVER_LABEL}, process_exit_codes::AI_CONFIG_READING_ERROR};
use bt_logger::{log_fatal, log_warning};
use bt_yaml_utils::{convert_yaml_to_vec_string, get_yaml};
use yaml_rust2::Yaml;

const AI_YML_CONFIG: &str = "config/ai/ai-config.yml";
const AI_YML_CONFIG_ENV_VAR_NAME: &str = "BT_AI_CONFIGYMLFILE";

const DEFAULT_NAME: &str = "BachuetechAI";
const DEFAULT_PORT: i64 = 11434;
const DEFAULT_MAX_CTX_SIZE: usize = 5;

#[derive(Debug)]
struct _AIServer {
    host: String,
    port: u16,
    secure: bool,
}

#[derive(Debug)]
struct AIApis {
    ctx_max: usize,
    path: String,
    chat: String,
    generate: String,
    models: String,
}

#[derive(Debug)]
pub struct AIConfig {
    name: String,
    platforms: HashMap<String, Platform>,
}


#[derive(Debug)]
struct Platform {
    api: AIApis,
    ai_url: String,
    models: HashMap<String, Model>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SupportedFunctions {
    ALL,
    NONE,
    Functions(Vec<String>),
}

impl SupportedFunctions {
    /// Method to convert a comma-separated string into a list of names
    fn from_str_list(s: &str) -> Self {
        let names: Vec<String> = s.split(',')
            .map(|name| name.trim().to_string()) // Trim each name and convert to String
            .collect();
        SupportedFunctions::Functions(names)
    }
}

impl From<String> for SupportedFunctions {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "ALL"  => SupportedFunctions::ALL,
            "NONE" => SupportedFunctions::NONE,
            _ => SupportedFunctions::from_str_list(&s), // Otherwise, treat it as a list of names
        }
    }
}

impl From<Yaml> for SupportedFunctions {
    fn from(s: Yaml) -> Self {
        match s.as_str() {
            Some("ALL") => SupportedFunctions::ALL,
            Some("NONE") => SupportedFunctions::NONE,
            _ => SupportedFunctions::Functions(convert_yaml_to_vec_string(&s)), // Otherwise, treat it as a list of names
        }
    }
}

#[derive(Debug)]
pub struct Model{
    pub model: String,
    pub tool_support: bool,
    pub system: String,
    pub tools: SupportedFunctions,
}
pub enum InteractionType {
    Chat,
    Generate,
    Models,
}

impl AIConfig {
    // Constructor to read from YAML file
    pub fn new(run_env: &String) -> Self {
        let ai_config: Yaml;
        match get_yaml(AI_YML_CONFIG_ENV_VAR_NAME,AI_YML_CONFIG){
            Ok(y_file_conf) => ai_config = y_file_conf,
            Err(e) => {
                log_fatal!("new","Fatal Error Reading AI configuration (PEC={}). Application aborted! {}",AI_CONFIG_READING_ERROR, e.to_string()); 
                process::exit(AI_CONFIG_READING_ERROR);
            }, // Exit the program with code -102 },,
        }

        let mut platform_list: HashMap<String, Platform> = HashMap::new();
        for plat in ai_config[run_env.as_str()][AI_PLATFORM_LABEL].clone() {

            let port = match plat[SERVER_LABEL][PORT_LABEL].as_i64(){
                Some(pn) => {
                    if pn < 0 || pn > 65535 {
                        log_warning!("new","Invalid Port Number {} in config file. Using default port {} instead",pn, DEFAULT_PORT);
                        DEFAULT_PORT
                    } else {
                        pn
                    }
                },
                None => {
                    log_warning!("new","Invalid Port Number {:?} in config file. Using default port {} instead",plat[SERVER_LABEL][PORT_LABEL], DEFAULT_PORT);
                    DEFAULT_PORT
                },
            };

            /*let port= if cfg_port < 0 || cfg_port > 65535 {
                log_warning!("new","Invalid Port Number {} in config file. Using default port {} instead",cfg_port, DEFAULT_PORT);
                DEFAULT_PORT
            } else {
                cfg_port
            };*/

            let host_data = _AIServer {
                host: plat[SERVER_LABEL][HOST_LABEL]
                    .as_str()
                    .unwrap_or("localhost")
                    .to_owned(),
                port: port as u16,
                secure: plat[SERVER_LABEL]["secure"].as_bool().unwrap_or(true),
            };

            let cfg_ctx_max; 
            match plat["api"]["ctx_max"].as_i64(){
                Some(cm) => {
                    if cm <= 0 {
                        log_warning!("new","Maximun Size of Context (ctx_max = {}) in AI YML config file is invalid. Using default value {}",cm, DEFAULT_MAX_CTX_SIZE);
                        cfg_ctx_max = DEFAULT_MAX_CTX_SIZE;
                    }else {
                        match usize::try_from(cm){
                            Ok(ucm) => cfg_ctx_max = ucm,
                            Err(e) => {
                                log_warning!("new","Error during conversion of Maximun Size of Context (ctx_max = {}) in AI YML config file. Using default value {}. Error: {}",cm, DEFAULT_MAX_CTX_SIZE, e);
                                cfg_ctx_max = DEFAULT_MAX_CTX_SIZE
                            },
                        };
                    }
                },
                None => {
                    log_warning!("new","Maximun Size of Context (ctx_max = {:?}) in AI YML config file is invalid. Using default value {}",plat["api"]["ctx_max"], DEFAULT_MAX_CTX_SIZE);
                    cfg_ctx_max = DEFAULT_MAX_CTX_SIZE;
                },
            }
            //let cfg_ctx_max = plat["api"]["ctx_max"].as_i64().unwrap_or(5);
            let api_data = AIApis {
                ctx_max: cfg_ctx_max,
                    //usize::try_from(plat["api"]["ctx_max"].as_i64().unwrap_or(5))
                    //.ok()
                    //.expect(get_fatal!("new","Maximun Size of Context (ctx_max = {:?}) in AI YML config file is invalid",plat["api"]["ctx_max"]).as_str()),
                path: plat["api"]["path"].as_str().unwrap_or("api").to_owned(),
                chat: plat["api"]["chat"].as_str().unwrap_or("chat").to_owned(),
                generate: plat["api"]["generate"].as_str().unwrap_or("generate").to_owned(),
                models: plat["api"]["models"].as_str().unwrap_or("models").to_owned(),
            };

            let mut url = format!("{}{}{}", host_data.host.clone(), ":", host_data.port);
            let end_point = format!("{}{}{}", "/", api_data.path.clone(), "/");

            if host_data.secure {
                url = format!("{}{}{}", "https://", url, end_point);
            } else {
                url = format!("{}{}{}", "http://", url, end_point);
            }

            let mut config_models: HashMap<String, Model> = HashMap::new();
            for m in plat["models"].clone() {
                config_models.insert(
                    m["model_id"].as_str().unwrap_or("default").to_owned(),
                    Model{
                        model: m["model"].as_str().unwrap_or(m["model_id"].as_str().unwrap_or("default")).to_owned(),
                        tool_support: m["tool_support"].as_bool().unwrap_or(false),
                        system: m["system"].as_str().unwrap_or("You are an AI assistance").to_owned(),
                        tools: SupportedFunctions::from(m["tools"].clone()),
                    },
                );
            }

            let p = Platform {
                api: api_data,
                ai_url: url,
                models: config_models,
            };

            platform_list.insert(
                plat["name"].as_str().unwrap_or("default").to_owned(),
                p,
            );

        }

        Self {
            name: ai_config["name"].as_str().unwrap_or(DEFAULT_NAME).to_owned(),
            platforms: platform_list,
        }
    }

    fn get_platform(&self, name: &String) -> Option<&Platform> {
        self.platforms.get(name)
    }

    pub fn get_name(&self) -> &String{
        &self.name
    }


    pub fn get_url(&self, platform_name: String, int_type: InteractionType) -> String {
        if let Some(p) = self.get_platform(&platform_name) {
            return match int_type {
                InteractionType::Chat => {
                    format!("{}{}", p.ai_url, p.api.chat.clone())
                }
                InteractionType::Generate => {
                    format!("{}{}", p.ai_url.clone(), p.api.generate.clone())
                }
                InteractionType::Models => {
                    format!("{}{}", p.ai_url.clone(), p.api.models.clone())
                }
            };
        } else {
            log_warning!("get_url","Platform {} NOT found. Using default values!",&platform_name);
            return match int_type { //Default Values!
                InteractionType::Chat => "http://localhost/default/chat".to_owned(),
                InteractionType::Generate => "http://localhost/default/generate".to_owned(),
                InteractionType::Models => "http://localhost/default/models".to_owned(),
            };
        }


    }

    pub fn get_platform_list(&self) -> Vec<String>{
        self.platforms.keys().cloned().collect()
    }

    pub fn get_models(&self, platform_name: &String) ->  Option<&HashMap<String, Model>>{
        if let Some(p) = self.get_platform(platform_name) {
            return Some(&p.models)
        }else{
            return None
        }
    }

    pub fn get_max_ctx_size(&self, platform_name: &String) -> usize {
        if let Some(p) = self.get_platform(platform_name) {
            p.api.ctx_max as usize
        }else{
            1
        }
    }

}

//**********/
//UNIT TEST
//*********/
#[cfg(test)]
mod tests_ai_config{
    use bt_logger::{build_logger, LogLevel, LogTarget};

    use crate::ai_config::InteractionType;

    use super::{AIConfig, SupportedFunctions};

    
    #[test]
    fn test_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let cfg = AIConfig::new(&"dev".to_string());
        assert_eq!(cfg.get_name(),"BT_AI");
        assert_eq!(cfg.get_platform_list().len(),2);
        assert_eq!(cfg.get_max_ctx_size(&"OLLAMALOCAL".to_string()),20);
        assert_eq!(cfg.get_url("OLLAMALOCAL".to_string(), InteractionType::Chat),"http://localhost:11434/api/chat");
        assert_eq!(cfg.get_url("OLLAMALOCAL".to_string(), InteractionType::Generate),"http://localhost:11434/api/generate");
        assert_eq!(cfg.get_url("OLLAMALOCAL".to_string(), InteractionType::Models),"http://localhost:11434/api/tags");
        assert_eq!(cfg.get_models(&"OLLAMALOCAL".to_string()).unwrap().len(),6);
    }

    #[test]
    fn test_wrong(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let cfg = AIConfig::new(&"dev".to_string());
         assert_eq!(cfg.get_max_ctx_size(&"wrong".to_string()),5);
        assert_eq!(cfg.get_url("wrong".to_string(), InteractionType::Chat),"http://127.0.0.1:11434/api/chat");
        assert_eq!(cfg.get_url("wrong".to_string(), InteractionType::Generate),"http://127.0.0.1:11434/api/generate");
        assert_eq!(cfg.get_url("wrong".to_string(), InteractionType::Models),"http://127.0.0.1:11434/api/tags");
        assert_eq!(cfg.get_models(&"wrong".to_string()).unwrap().len(),0);
    }

    #[test]
    fn test_not_found_success(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let cfg = AIConfig::new(&"UNKNOWN".to_string());  
        assert_eq!(cfg.get_name(),"BT_AI");
        assert_eq!(cfg.get_platform_list().len(),0);  
        assert_eq!(cfg.get_max_ctx_size(&"UNKNOWN".to_string()),1);
        assert_eq!(cfg.get_url("UNKNOWN".to_string(), InteractionType::Chat),"http://localhost/default/chat");
        assert_eq!(cfg.get_url("UNKNOWN".to_string(), InteractionType::Generate),"http://localhost/default/generate");
        assert_eq!(cfg.get_url("UNKNOWN".to_string(), InteractionType::Models),"http://localhost/default/models");        
        assert!(cfg.get_models(&"UNKNOWN".to_string()).is_none());
    }

    #[test]
    fn test_supp_funct_all_none(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        assert_eq!(SupportedFunctions::from("All".to_string()), SupportedFunctions::ALL);
        assert_eq!(SupportedFunctions::from("None".to_string()),SupportedFunctions::NONE);
    }

    #[test]
    fn test_supp_funct_func(){
        build_logger("BACHUETECH", "BT.AI_CONFIG", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let ffn = "Fake_func".to_owned();
        let f = SupportedFunctions::from(ffn.clone());
        let sf;
        match f {
            SupportedFunctions::Functions(items) => sf = items[0].clone(),
            _ => sf = "Error".to_owned(),
        }
        assert_eq!(sf,ffn);
        
    }

}

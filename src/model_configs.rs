use std::{collections::HashMap, path::PathBuf};

use bt_logger::get_error;
use bt_string_utils::{remove_char, RemoveLocationEnum};
use bt_yaml_utils::{get_f32, get_u32, get_usize, get_yaml};
use rand::Rng;
use yaml_rust2::Yaml;

use crate::{ai_config::SupportedFunctions, parameter_names::{FRAMEWORK_MODEL_DISABLE_GPU, SAMPLER_PENALTY_LAST_N, SAMPLER_PENALTY_REPEAT, SAMPLER_SEED, SAMPLER_TEMP, SAMPLER_TOP_K, SAMPLER_TOP_P}};

#[derive(Clone, Debug)]
pub struct ModelConfig{
    model_root_folder: String,
    model_path: String, /// The path to the model
    system: String,
    tools: SupportedFunctions,    
    ctx_params: HashMap<String,Yaml>,
    model_params: HashMap<String,Yaml>,
    sampler_params: HashMap<String,Yaml>,
    model_cfg_parms: HashMap<String,String>,
}

#[derive(Clone)]
pub struct ModelConfigs{
    models: HashMap<String, ModelConfig>,
}

const LLAMA_MODEL_YML_CONFIG: &str = "config/model-cfg.yml";
const LLAMA_MODEL_YML_CONFIG_ENV_VAR_NAME: &str = "BT_LLAMAMODEL_CONFIGYMLFILE";
const DEFAULT_ROOT_MODEL_FOLDER: &str = "models";

impl ModelConfigs{
    // Constructor to read from YAML file
    pub fn new(run_env: &str) -> Result<Self, String> {
        let llama_model_cfg = 
                                get_yaml(LLAMA_MODEL_YML_CONFIG_ENV_VAR_NAME,LLAMA_MODEL_YML_CONFIG)
                                .map_err(|e| get_error!("new","Error reading Model Configuation File. Error {}",e))?;
        let root_folder = remove_char(RemoveLocationEnum::End, 
                                            &llama_model_cfg[run_env]["root_folder"].as_str().unwrap_or(DEFAULT_ROOT_MODEL_FOLDER).to_owned(),
                                            '/');
        let mut models: HashMap<String, ModelConfig> = HashMap::new();
        let model_list = llama_model_cfg[run_env]["models"].clone();
        for m in model_list {
            let mut v_ctx_params: HashMap<String, Yaml> = HashMap::new();
            let ctx_param_list = m["ctx_params"].clone();
            for cp in ctx_param_list{
                v_ctx_params.insert(cp["param_id"].as_str().unwrap_or("UNKNOWN").to_owned(),
                                    cp["param_value"].clone() );
            }
            let mut v_model_params: HashMap<String, Yaml> = HashMap::new();
            let model_param_list = m["model_params"].clone();
            for mp in model_param_list{
                v_model_params.insert(mp["param_id"].as_str().unwrap_or("UNKNOWN").to_owned(),
                                    mp["param_value"].clone() );
            }
            let mut v_sampler_params: HashMap<String,Yaml> = HashMap::new();
            let sampler_param_list = m["sampler_params"].clone();
            for sp in sampler_param_list{
                v_sampler_params.insert(sp["param_id"].as_str().unwrap_or("UNKNOWN").to_owned(),
                                        sp["param_value"].clone());
            }
            models.insert(m["model_id"].as_str().unwrap_or("default").to_owned(),
            ModelConfig{
                model_root_folder: root_folder.clone(),
                model_path: m["model_path"].as_str().unwrap_or("qwen3:latest").to_string(),
                system: m["system"].as_str().unwrap_or("").to_owned(),
                tools: SupportedFunctions::from(m["tools"].clone()),                
                ctx_params: v_ctx_params,
                model_params: v_model_params,
                sampler_params: v_sampler_params,
                model_cfg_parms: HashMap::new(),
            });
        }

        Ok(Self{
            models,
        })
    }    


    
    pub fn get_model_configs (&self, model_id: &str) -> Option<ModelConfig>{
        match self.models.get(model_id) {
            Some(lmc) => Some(lmc.clone()),
            None => None,
        }
    }
}

impl ModelConfig {
    pub fn set_custom_model_cfg_param(&mut self, parameter_id: &str, parameter_value: String) -> Option<String>{
        self.model_cfg_parms.insert(parameter_id.trim().to_lowercase(), parameter_value)
    }

    pub fn get_model_file_path(&self) -> PathBuf{
        PathBuf::from(format!("{}/{}",self.model_root_folder,self.model_path))
    }

    pub fn get_custom_model_cfg_param(&self, parameter_id: &str) -> Option<&String>{
        self.model_cfg_parms.get(parameter_id)
    }

    pub fn get_path(&self) -> String{
        self.model_path.clone()
    }

    pub fn get_system(&self) -> Option<String>{
        Some(self.system.clone())
    }

    pub fn get_ctx_parameters(&self) -> Option<HashMap<String,Yaml>>{
            Some(self.ctx_params.clone())
    }

    pub fn get_ctx_param(&self, param_id: &str) -> Option<Yaml>{
           self.ctx_params.get(param_id).cloned()
    }

    pub fn get_model_parameters(&self) -> Option<HashMap<String,Yaml>>{
            Some(self.model_params.clone())
    }

    pub fn get_model_param(&self, param_id: &str) -> Option<Yaml>{
           self.model_params.get(param_id).cloned()
    } 

    pub fn get_sampler_parameters(&self) -> Option<HashMap<String,Yaml>>{
            Some(self.sampler_params.clone())
    }

    pub fn get_sampler_param(&self, param_id: &str) -> Option<Yaml>{
            self.sampler_params.get(param_id).cloned()
    }

    pub fn get_sampler_temperature(&self) -> Option<f64> {
        self.get_sampler_param(SAMPLER_TEMP).and_then(|v| v.as_f64())
    } 

    pub fn get_sampler_top_p(&self) -> Option<f64> {
        self.get_sampler_param(SAMPLER_TOP_P).and_then(|v| v.as_f64())
    }  

    pub fn get_sampler_top_k(&self) -> Option<usize> {
        self.get_sampler_param(SAMPLER_TOP_K).and_then(|v| v.as_i64()).and_then(|tk| Some(tk as usize))
    }

    ///Get initial seed as u32. 
    ///u32 to keep Compatibility with Llama.CPP, just in case
    fn get_ini_seed() -> u32{
        let mut rng = rand::rng();
        rng.random_range(1..=u32::MAX)
    }

    pub fn get_sampler_seed(&self) -> u32 {
        get_u32(self.get_sampler_param(SAMPLER_SEED).as_ref(), Self::get_ini_seed())
    }        
        
    pub fn get_sampler_repeat_penalty(&self) -> f32 {
        get_f32(self.get_sampler_param(SAMPLER_PENALTY_REPEAT).as_ref(), 1.25)
    }

    pub fn get_sampler_repeat_last_n(&self) -> usize {
        get_usize(self.get_sampler_param(SAMPLER_PENALTY_LAST_N).as_ref(), 128)
    }   

    pub fn get_model_disbale_gpu(&self) -> bool {
        self.get_model_param(FRAMEWORK_MODEL_DISABLE_GPU).map(|v| v.as_bool().unwrap_or(false)).unwrap_or(false)
        //get_usize(self.get_sampler_param(SAMPLER_PENALTY_LAST_N).as_ref(), 128)
    }
}

//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod model_config_tests {
    use bt_logger::{build_logger, log_verbose, LogLevel, LogTarget};
    use super::ModelConfigs;

    #[cfg(test)]
    const VALID_ENV: &str = "dev";

    #[cfg(test)]
    const VALID_MODEL_ID: &str = "llama3.1:latest";

    #[test]
    pub fn test_get_model_path(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR, None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_path();
        assert_eq!(p,"/home/super/.ollama/models/blobs/sha256-e2f46f5b501c2982b2c495a4694cb4e620aabfa2c37ebb23a90ffc8cce93854b");
    }

    #[test]
    pub fn test_get_system(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR, None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_system().unwrap();
        assert_eq!(p,"You are Jeremy. You are an AI assistant with tool calling capabilities");
    }    

    #[test]
    pub fn test_get_ctx_parameters(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_ctx_parameters().unwrap();
        log_verbose!("test_get_ctx_param","ctx = {:?}",p);
        assert_eq!(p.get("n_ctx").unwrap().as_i64().unwrap(), 4096);
        assert_eq!(p.len(),3);
    }  

    #[test]
    pub fn test_get_ctx_param(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_ctx_param("n_ctx").unwrap();
        assert_eq!(p.as_i64().unwrap(), 4096);
    }      

    #[test]
    pub fn test_get_model_parameters(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_model_parameters().unwrap();
        log_verbose!("test_get_model_parameters","mod = {:?}",p);        
        assert_eq!(p.get("disable_gpu").unwrap().as_bool().unwrap(), false);
        assert_eq!(p.len(),4);
    }  

    #[test]
    pub fn test_model_ctx_param(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_model_param("disable_gpu").unwrap();
        assert_eq!(p.as_bool().unwrap(), false);
    }

    #[test]
    pub fn test_get_sampler_parameters(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_sampler_parameters().unwrap();
        log_verbose!("test_get_sampler_parameters","sampler = {:?}",p);            
        assert_eq!(p.get("temperature").unwrap().as_f64().unwrap(), 1.0);
        assert_eq!(p.len(),8);
    }  

    #[test]
    pub fn test_sampler_ctx_param(){
        build_logger("BACHUETECH","SERVER_CONFIG",LogLevel::VERBOSE,LogTarget::STD_ERROR,None);
        let c = ModelConfigs::new(&VALID_ENV.to_owned()).unwrap().get_model_configs(VALID_MODEL_ID).unwrap();
        let p = c.get_sampler_param("temperature").unwrap();
        log_verbose!("test_sampler_ctx_param", "temp {:?}", p);
        assert_eq!(p.as_f64().unwrap(), 1.0);
    }       
}
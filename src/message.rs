use serde::{Deserialize, Serialize};

use crate::ai_tool_to_call::ToolToCall;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole{
    USER,
    ASSISTANT,
    SYSTEM,
    TOOL,
    IPYTHON,
    ERROR,
}

impl MessageRole {
    fn as_str(&self) -> &'static str {
        match self{
            MessageRole::USER => "user",
            MessageRole::ASSISTANT => "assistant",
            MessageRole::SYSTEM => "system",
            MessageRole::TOOL => "tool",
            MessageRole::IPYTHON => "ipython",
            MessageRole::ERROR => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message{
    role: MessageRole,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolToCall>>,
}

impl Message{
    pub fn new(role: MessageRole,  msg_content: String) -> Self{
        Message{
            role: role,
            content: msg_content,
            tool_calls: None,
        }
    }

    pub fn new_with_tools(role: MessageRole,  msg_content: String, tools: Vec<ToolToCall>) -> Self{
        Message{
            role: role,
            content: msg_content,
            tool_calls: Some(tools),
        }
    }

    /*pub fn new_from_json(json_message: String) -> Self{
        serde_json::from_str(&json_message).unwrap()
    }*/

    pub fn get_content(&self) -> &String{
        &self.content
    }

    pub fn get_role(&self) -> &MessageRole{
        &self.role
    }

    /*pub fn get_message_json(&self) -> String{
         serde_json::to_string(self).unwrap()
    }*/

    pub fn get_tools(&self) -> Option<Vec<ToolToCall>> {
        self.tool_calls.clone()
    }
}

//**********/
//UNIT TEST
//*********/
#[cfg(test)]
mod tests_message{
    use std::collections::HashMap;
    use serde_json::Value;
    use crate::ai_tool_to_call::ToolToCall;

    use super::{Message, MessageRole};

    #[test]
    fn test_message_success(){
        let content: String = "This is a prompt".to_owned();
        let msg = Message::new(MessageRole::USER, content.clone());
        assert_eq!(msg.get_role().clone(),MessageRole::USER);
        assert_eq!(msg.get_content().clone(),content);
        assert!(msg.get_tools().is_none());
    }

    #[test]
    fn test_message_with_funct(){
        let ctt: String = "This is a prompt".to_owned();
        let mut arg: HashMap<String, Value> = HashMap::new();
        arg.insert("ARG1".to_owned(), Value::String("Value1".to_owned()));

        let ttc = ToolToCall::new("FunctName".to_owned(), arg);
  

        let msg = Message{
            role: MessageRole::USER,
            content: ctt.clone(),
            tool_calls: Some(vec![ttc]),
        };

        assert_eq!(msg.get_role().clone(),MessageRole::USER);
        assert_eq!(msg.content,ctt);
        assert_eq!(msg.get_tools().unwrap()[0].get_function_name(),"FunctName");
    }

    #[test]
    fn test_as_str() {
        assert_eq!(MessageRole::USER.as_str(), "user");
        assert_eq!(MessageRole::ASSISTANT.as_str(), "assistant");
        assert_eq!(MessageRole::SYSTEM.as_str(), "system");
        assert_eq!(MessageRole::TOOL.as_str(), "tool");
        assert_eq!(MessageRole::IPYTHON.as_str(), "ipython");
        assert_eq!(MessageRole::ERROR.as_str(), "error");
    }

}
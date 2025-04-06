use bt_logger::{log_error, log_trace};
use serde::{Deserialize, Serialize};

use crate::{
    ai_tools::Tool,
    message::{Message, MessageRole},
};

#[derive(Serialize)]
pub struct AIChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AIChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done_reason: String,
    pub done: bool,
    pub total_duration: u128,
    pub load_duration: u128,
    pub prompt_eval_count: u64,
    pub prompt_eval_duration: u128,
    pub eval_count: u64,
    pub eval_duration: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AIChatBodyMessage {
    pub message: Message,
    pub context: Vec<Message>,
    pub done: bool,
}

/*pub fn get_chat_request( ai_model: &String, role: MessageRole, message: &String, context: Vec<Message>, system: Option<String>, tool_list: Option<Vec<Tool>>, 
                        current_date: &str, current_time: &str, ) -> String {
    let ai_request = get_chat_ai_chat_request(ai_model, role, message, context, system, tool_list, current_date, current_time);
    get_chat_request_json(&ai_request)
}*/

pub fn get_chat_request_json(ai_request: &AIChatRequest) -> String{
    match serde_json::to_string(&ai_request) {
        Ok(sj) => {
            return sj;
        }
        Err(e) => {
            let bem = format!("{{\"model\": \"{}\", \"message\": \"{:?}\", \"stream\": false}}",&ai_request.model, &ai_request.messages );
            log_error!( "get_chat_request", "Error creating JSON Request. Returning default message as a best effort with no tools: {}. Error: {}", &bem, e );
            return bem;
        }
    }
}

pub fn get_chat_ai_chat_request( ai_model: &String, role: MessageRole, message: &String, context: Vec<Message>, system: Option<String>, tool_list: Option<Vec<Tool>>, 
                                current_date: &str, current_time: &str, stream_ans: bool ) -> AIChatRequest {
    log_trace!( "model_chat", "Ready to start chat role {:?}: {}", &role, &message );

    let mut initial_msg: Vec<Message> = Vec::new();
    if let Some(sys_msg) = system {
        initial_msg.push(Message::new(
            MessageRole::SYSTEM,
            format!("{}. The current date is {} and the current time is {}", sys_msg, &current_date, &current_time),
        ));
    }
    initial_msg.extend(context.clone()); //payload.context.clone());
    let user_message = Message::new(role, message.to_string());
    initial_msg.push(user_message.clone()); //Needed Later to build the context (history)

    AIChatRequest {
        model: ai_model.to_owned(),
        messages: initial_msg.clone(),
        stream: stream_ans,
        tools: tool_list.clone(),
    }
}

//**********/
//UNIT TEST
//*********/
#[cfg(test)]
mod tests_ai_config {
    use bt_logger::{LogLevel, LogTarget, build_logger};

    use crate::{ai_chat_helper::{get_chat_ai_chat_request, get_chat_request_json}, message::MessageRole};

    #[test]
    fn test_chat_req_success() {
        build_logger(
            "BACHUETECH",
            "BT.AI_CHAT_HELPER",
            LogLevel::VERBOSE,
            LogTarget::STD_ERROR,
        );
        //let resp = get_chat_request(
            let resp = get_chat_ai_chat_request(
            &"llama3.1".to_string(),
            MessageRole::USER,
            &"The prompt".to_string(),
            Vec::new(),
            Some("AI Assistant".to_owned()),
            None,
            "03/27/2025",
            "6:45 PM",
            false
        );
        let json_resp = get_chat_request_json(&resp);
        let json_a = "{\"model\":\"llama3.1\",\"messages\":[{\"role\":\"system\",\"content\":\"AI Assistant. The current date is 03/27/2025 and the current time is 6:45 PM\"},{\"role\":\"user\",\"content\":\"The prompt\"}],\"stream\":false}";
        println!("MSG: {}", &json_resp);
        assert_eq!(json_resp, json_a);
    }
}

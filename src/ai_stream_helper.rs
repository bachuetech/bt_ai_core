use std::{thread, time};

use bt_http_utils::{stream_response::HttpStreamResponse, HttpResponse};
use bt_logger::{log_error, log_verbose};

use crate::{ai_chat_helper::AIChatResponse, ai_tool_to_call::ToolToCall, message::{Message, MessageRole}};

const MAX_NUM_ERRORS: i8 = 5;

pub async fn process_stream(mut streamer: HttpStreamResponse) -> HttpResponse{
    let mut streamed_content = String::new();
    let mut streamed_tools: Option<Vec<ToolToCall>> = None;
    let mut streamer_remote_address: String = "0.0.0.0".to_owned();
    let mut error_count = 0;

    let mut last_response = AIChatResponse{
        model: "UNKNOWN_MODEL_ERROR".to_owned(),
        created_at: "".to_owned(),
        message: Message::new(MessageRole::ERROR, "NO RESPONSE ERROR!".to_owned()),
        done_reason: Some("error".to_owned()),
        done: true,
        total_duration: Some(0),
        load_duration: Some(0),
        prompt_eval_count: Some(0),
        prompt_eval_duration: Some(0),
        eval_count: Some(0),
        eval_duration: Some(0),
    };

    log_verbose!("process_stream","Ready to Stream!");
    while let Some(int_http_resp) = streamer.read_stream().await {
        streamer_remote_address = int_http_resp.remote_address;
        match serde_json::from_str(&int_http_resp.body){
            Ok(r) => {
                last_response = r;
                streamed_content.push_str(last_response.message.get_content());

                if streamed_tools.is_none(){
                    streamed_tools = last_response.message.get_tools();
                }
            },
            Err(e) => {
                if error_count > MAX_NUM_ERRORS {
                    log_error!("process_stream", "Too many failures (>{}) converting JSON body. Abort reading/conversion. Error: {}", MAX_NUM_ERRORS,e);
                    break
                }
                error_count += 1;
                log_error!("process_stream", "Fail to convert JSON body. Waiting {} milliseconds before continue. Error: {}",&error_count, e);
                thread::sleep(time::Duration::from_millis(error_count.try_into().unwrap()));
            },
        }
    }

    let msg: Message;
    if let Some(t ) = streamed_tools{
        msg = Message::new_with_tools(last_response.message.get_role().clone(), streamed_content, t);
    }else{
        msg = Message::new(last_response.message.get_role().clone(), streamed_content);
    }

    let cr = AIChatResponse{
        model: last_response.model,
        created_at: last_response.created_at,
        message: msg,
        done_reason: last_response.done_reason,
        done: last_response.done,
        total_duration: last_response.total_duration,
        load_duration: last_response.load_duration,
        prompt_eval_count: last_response.prompt_eval_count,
        prompt_eval_duration: last_response.prompt_eval_duration,
        eval_count: last_response.eval_count,
        eval_duration: last_response.eval_duration,
    };

    //let j_body: String;

    log_verbose!("process_stream", "Convert to JSON");
    let j_body: String = match serde_json::to_string(&cr){
        Ok(j) =>  j,
        Err(e) => {
            log_error!("process_stream","Body {:?} cannot be converted to JSON. Error {}",&cr,e);
            "".to_owned()
        },
    };

    HttpResponse{
        status_code: streamer.get_status(),
        header: streamer.get_ini_header(),
        body: j_body,
        remote_address: streamer_remote_address,
    }
}


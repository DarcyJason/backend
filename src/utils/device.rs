use axum::http::HeaderMap;
use user_agent_parser::{Device, UserAgentParser};

pub fn get_device_info(headers: HeaderMap) -> String {
    let user_agent = headers
        .get("User-Agent")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let ua_parser = UserAgentParser::from_path("regex.yaml").unwrap();
    let device = ua_parser.parse_device(user_agent.as_str());
    match device {
        Device {
            name: Some(name),
            brand: Some(brand),
            model: Some(model),
        } => {
            format!("{} {} {}", name, brand, model)
        }
        _ => "Unkwown".to_string(),
    }
}

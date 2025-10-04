use user_agent_parser::{Device, UserAgentParser};

pub fn get_device_info(user_agent: String) -> String {
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

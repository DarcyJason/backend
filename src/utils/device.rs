use uaparser_rs::UAParser;

pub fn parse_user_agent_detailed(user_agent: &str) -> (String, String, String) {
    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let client = uap.parse(user_agent);
    let user_agent = format!(
        "{} {}.{}.{}.{}",
        client.user_agent.family,
        client.user_agent.major.unwrap_or("Unknown".to_string()),
        client.user_agent.minor.unwrap_or("Unknown".to_string()),
        client.user_agent.patch.unwrap_or("Unknown".to_string()),
        client
            .user_agent
            .patch_minor
            .unwrap_or("Unknown".to_string()),
    );
    let os = format!(
        "{} {}.{}.{}",
        client.os.family,
        client.os.major.unwrap_or("Unknown".to_string()),
        client.os.minor.unwrap_or("Unknown".to_string()),
        client.os.patch.unwrap_or("Unknown".to_string()),
    );
    let device = format!(
        "{}: {} {}",
        client.device.family,
        client.device.brand.unwrap_or("Unknown".to_string()),
        client.device.model.unwrap_or("Unknown".to_string()),
    );
    (user_agent, os, device)
}

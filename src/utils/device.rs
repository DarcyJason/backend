use uaparser_rs::UAParser;

pub fn parse_user_agent_detailed(
    user_agent: &str,
) -> (
    String, // user_agent_family
    String, // user_agent_major
    String, // user_agent_minor
    String, // user_agent_patch
    String, // user_agent_patch_minor
    String, // os_family
    String, // os_major
    String, // os_minor
    String, // os_patch
    String, // device_family
    String, // device_brand
    String, // device_model
) {
    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let client = uap.parse(user_agent);
    (
        client.user_agent.family,
        client.user_agent.major.unwrap_or("Unknown".to_string()),
        client.user_agent.minor.unwrap_or("Unknown".to_string()),
        client.user_agent.patch.unwrap_or("Unknown".to_string()),
        client
            .user_agent
            .patch_minor
            .unwrap_or("Unknown".to_string()),
        client.os.family,
        client.os.major.unwrap_or("Unknown".to_string()),
        client.os.minor.unwrap_or("Unknown".to_string()),
        client.os.patch.unwrap_or("Unknown".to_string()),
        client.device.family,
        client.device.brand.unwrap_or("Unknown".to_string()),
        client.device.model.unwrap_or("Unknown".to_string()),
    )
}

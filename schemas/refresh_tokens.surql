DEFINE TABLE OVERWRITE refresh_tokens SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON refresh_tokens;
DEFINE FIELD OVERWRITE id ON refresh_tokens TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON refresh_tokens TYPE record<users>;
DEFINE FIELD OVERWRITE device_id ON refresh_tokens TYPE record<devices>;
DEFINE FIELD OVERWRITE token_value ON refresh_tokens TYPE string;
DEFINE FIELD OVERWRITE created_at ON refresh_tokens TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON refresh_tokens TYPE datetime DEFAULT time::now() + 7d;

DEFINE INDEX OVERWRITE refresh_token_user_id_index ON TABLE refresh_tokens COLUMNS user_id;
DEFINE INDEX OVERWRITE refresh_token_device_id_index ON TABLE refresh_tokens COLUMNS device_id;
DEFINE INDEX OVERWRITE refresh_token_value_index ON TABLE refresh_tokens COLUMNS token_value UNIQUE;

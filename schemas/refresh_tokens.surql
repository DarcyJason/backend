DEFINE TABLE OVERWRITE refresh_tokens SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON refresh_tokens;
DEFINE FIELD OVERWRITE id ON refresh_tokens TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON refresh_tokens TYPE string;
DEFINE FIELD OVERWRITE token_value ON refresh_tokens TYPE string;
DEFINE FIELD OVERWRITE is_revoked ON refresh_tokens TYPE bool;
DEFINE FIELD OVERWRITE created_at ON refresh_tokens TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expired_at ON refresh_tokens TYPE datetime DEFAULT time::now();

DEFINE INDEX refresh_token_user_id_index ON TABLE refresh_tokens COLUMNS user_id;
DEFINE INDEX refresh_token_value_index ON TABLE refresh_tokens COLUMNS token_value UNIQUE;

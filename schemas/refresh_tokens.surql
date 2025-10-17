DEFINE TABLE OVERWRITE refresh_tokens SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON refresh_tokens;
DEFINE FIELD OVERWRITE id ON refresh_tokens TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON refresh_tokens TYPE string;
DEFINE FIELD OVERWRITE token ON refresh_tokens TYPE string;
DEFINE FIELD OVERWRITE created_at ON refresh_tokens TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE expires_at ON refresh_tokens TYPE datetime DEFAULT time::now();

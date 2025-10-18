DEFINE TABLE OVERWRITE email_verification_tokens SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON email_verification_tokens;
DEFINE FIELD OVERWRITE id ON email_verification_tokens Type uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON email_verification_tokens Type string;
DEFINE FIELD OVERWRITE token ON email_verification_tokens Type string;
DEFINE FIELD OVERWRITE token_Type ON email_verification_tokens Type string;
DEFINE FIELD OVERWRITE created_at ON email_verification_tokens Type datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON email_verification_tokens Type datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE used ON email_verification_tokens Type bool DEFAULT false;

DEFINE INDEX OVERWRITE email_verification_token_index ON TABLE email_verification_tokens COLUMNS token UNIQUE;

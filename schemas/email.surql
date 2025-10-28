DEFINE TABLE OVERWRITE email SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON emails;
DEFINE FIELD OVERWRITE id ON emails TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON emails TYPE record<users>;
DEFINE FIELD OVERWRITE email_type ON emails TYPE string;
DEFINE FIELD OVERWRITE email_token ON emails TYPE string;
DEFINE FIELD OVERWRITE created_at ON emails TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON emails TYPE datetime DEFAULT time::now() + 1d;
DEFINE FIELD OVERWRITE is_used ON emails TYPE bool DEFAULT false;

DEFINE INDEX OVERWRITE email_index ON TABLE emails COLUMNS email_token UNIQUE;

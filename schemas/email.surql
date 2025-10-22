DEFINE TABLE OVERWRITE email SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON email;
DEFINE FIELD OVERWRITE id ON email Type uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON email Type string;
DEFINE FIELD OVERWRITE token_type ON email Type string;
DEFINE FIELD OVERWRITE email_token ON email Type string;
DEFINE FIELD OVERWRITE created_at ON email Type datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON email Type datetime DEFAULT time::now() + 1d;

DEFINE INDEX OVERWRITE email_index ON TABLE email COLUMNS token UNIQUE;

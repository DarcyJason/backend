DEFINE TABLE OVERWRITE devices SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON devices;
DEFINE FIELD OVERWRITE id ON devices TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON devices TYPE string;
DEFINE FIELD OVERWRITE device ON devices TYPE string;
DEFINE FIELD OVERWRITE ip ON devices TYPE string;
DEFINE FIELD OVERWRITE last_login_at ON devices TYPE datetime DEFAULT time::now();

DEFINE INDEX device_user_id_index ON TABLE devices COLUMNS user_id;

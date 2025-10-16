DEFINE TABLE OVERWRITE trusted_devices SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON trusted_devices;
DEFINE FIELD OVERWRITE id ON trusted_devices TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE user_id ON trusted_devices TYPE string;
DEFINE FIELD OVERWRITE device ON trusted_devices TYPE string;
DEFINE FIELD OVERWRITE ip ON trusted_devices TYPE string;
DEFINE FIELD OVERWRITE last_login_at ON trusted_devices TYPE datetime DEFAULT time::now();


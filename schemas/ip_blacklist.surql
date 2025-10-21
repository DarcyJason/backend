DEFINE TABLE OVERWRITE ip_blacklist SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON ip_blacklist;
DEFINE FIELD OVERWRITE id ON ip_blacklist TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE ip ON ip_blacklist TYPE string;
DEFINE FIELD OVERWRITE reason ON ip_blacklist TYPE string;
DEFINE FIELD OVERWRITE created_at ON ip_blacklist TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE expires_at ON ip_blacklist TYPE datetime;

DEFINE INDEX OVERWRITE ip_blacklist_id_index ON TABLE ip_blacklist COLUMNS id;
DEFINE INDEX OVERWRITE ip_blacklist_ip_index ON TABLE ip_blacklist COLUMNS ip;

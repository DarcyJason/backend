DEFINE TABLE OVERWRITE audit_log SCHEMALESS;

# DEFINE FIELD OVERWRITE field ON audit_log;
DEFINE FIELD OVERWRITE id ON audit_log TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE actor ON audit_log TYPE string;
DEFINE FIELD OVERWRITE action ON audit_log TYPE string;
DEFINE FIELD OVERWRITE status ON audit_log TYPE string;
DEFINE FIELD OVERWRITE timestamp ON audit_log TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD OVERWRITE ip ON audit_log TYPE option<string>;
DEFINE FIELD OVERWRITE user_agent ON audit_log TYPE option<string>;
DEFINE FIELD OVERWRITE details ON audit_log TYPE option<object>;
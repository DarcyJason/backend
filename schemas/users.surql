DEFINE TABLE OVERWRITE users SCHEMALESS;

DEFINE FIELD OVERWRITE id ON users TYPE uuid DEFAULT rand::uuid::v4();
DEFINE FIELD OVERWRITE name ON users TYPE string;
DEFINE FIELD OVERWRITE email ON users TYPE string;
DEFINE FIELD OVERWRITE password ON users TYPE string;
DEFINE FIELD OVERWRITE salt ON users TYPE string;
DEFINE FIELD OVERWRITE is_active ON users TYPE bool;
DEFINE FIELD OVERWRITE created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON users TYPE datetime DEFAULT time::now();

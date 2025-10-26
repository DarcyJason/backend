GET http://localhost:7878/api/v1/health
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.code" == 200
jsonpath "$.message" == "UP"
jsonpath "$.data" == null

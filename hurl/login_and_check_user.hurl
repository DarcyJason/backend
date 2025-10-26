POST http://localhost:7878/api/v1/auth/login
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/536.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36
Content-Type: application/json
{
    "email": "test@example.com",
    "password": "83ICqbyCZ@KDOd6Ip"
}
HTTP 200
[Captures]
auth_header_value: header "Authorization"
refresh_token: cookie "refresh_token[Value]"

GET http://localhost:7878/api/v1/user/me
Authorization: {{auth_header_value}}

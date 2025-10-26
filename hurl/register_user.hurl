POST http://localhost:7878/api/v1/auth/register
{
    "name": "test",
    "email": "test@example.com",
    "password": "83ICqbyCZ@KDOd6Ip",
    "confirm_password": "83ICqbyCZ@KDOd6Ip"
}

POST http://localhost:7878/api/v1/auth/login
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36
{
    "email": "test@example.com",
    "password": "83ICqbyCZ@KDOd6Ip"
}

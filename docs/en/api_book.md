# API Book

## Content

[API](#api)

## API

### Health check API

```
GET http://localhost:7878/api/v1/health
```

### Register user API

```
POST http://localhost:7878/api/v1/auth/register
{
    "name": "<your name>",
    "email": "<your email>",
    "password": "<your password>",
    "confirm_password": "<your password>"
}
```

### Login user API

```
POST http://localhost:7878/api/v1/auth/login
User-Agent: <your user agent>
{
    "email": "<your email>",
    "password": "<your password>"
}
```

### Logout user API

```
POST http://localhost:7878/api/v1/auth/logout
Authorization: Bearer <your access token>
Cookie: refresh_token=<your refresh token>
User-Agent: <your user agent>
```

### Verify user API

You can get the token from [resend](https://resend.com/emails) or the user's email.
```
POST http://localhost:7878/api/v1/auth/verify
User-Agent: <your user agent>
{
    "email": "test@example.com",
    "token": "<your token>"
}
```

### Get user information API

```
GET http://localhost:7878/api/v1/user/me
Authorization: Bearer <your access token>
Cookie: refresh_token=<your refresh token>
User-Agent: <your user agent>
```

# API Book

## Content
[Preparation](#preparation)

[API](#api)

[Test API](#test-api)

## Preparation

### Install hurl command

```sh
cargo install --locked hurl
```

### Configure the parameters in hurl file or use the default values

## Change directory to hurl directory

Please make sure you are in hurl directory or use the following command in the root of the project:

```sh
cd hurl
```

## API

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

if user is a new user, use this api

```
POST http://localhost:7878/api/v1/auth/login
{
    "email": "<your email>",
    "password": "<your password>"
}
```

if user is verified, use this api, this will return access_token in `Authorization` header and refresh_token in Cookie.

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

## Test API

### Register user

```sh
hurl register_user.hurl
```

### Verify user

```sh
hurl verify_user.hurl --variable token=<The token in the email>
```

### Login user and check user function

```sh
hurl login_and_check_user.hurl
```

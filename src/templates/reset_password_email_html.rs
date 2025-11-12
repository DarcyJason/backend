pub const RESET_PASSWORD_EMAIL_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Reset Your Password</title>
            <style>
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol';
                    background-color: #f4f4f4;
                    margin: 0;
                    padding: 0;
                    -webkit-font-smoothing: antialiased;
                }
                .container {
                    max-width: 600px;
                    margin: 20px auto;
                    background-color: #ffffff;
                    padding: 30px;
                    border-radius: 8px;
                    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
                }
                .header {
                    border-bottom: 1px solid #e9e9e9;
                    padding-bottom: 20px;
                    margin-bottom: 20px;
                    text-align: center;
                }
                .header h1 {
                    color: #333333;
                    font-size: 24px;
                    margin: 0;
                }
                .content p {
                    color: #555555;
                    line-height: 1.6;
                }
                .token-display {
                    background-color: #f0f0f0;
                    padding: 15px;
                    margin: 20px 0;
                    text-align: center;
                    border-radius: 4px;
                }
                .token-display code {
                    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
                    font-size: 18px;
                    color: #333;
                    letter-spacing: 2px;
                }
                .footer {
                    margin-top: 20px;
                    text-align: center;
                    font-size: 12px;
                    color: #999999;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>Reset Your Password</h1>
                </div>
                <div class="content">
                    <p>Hello, {{username}}!</p>
                    <p>We received a request to reset your password. Please use the following token to set a new password:</p>
                    <div class="token-display">
                        <code>{{email_token}}</code>
                    </div>
                    <p>This token will expire in 30 minutes.</p>
                    <p>If you did not request a password reset, please ignore this email.</p>
                </div>
                <div class="footer">
                    <p>Best regards,<br>The Application Team</p>
                </div>
            </div>
        </body>
    </html>
"#;

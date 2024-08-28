# Auto sender

To make the `mail_attachement.py` work, you need to create a file `credentials.py` containing:

```py
SMTP_SERVER = "smtp.gmail.com"
PORT = 587
SENDER_EMAIL = "your.email@gmail.com"
PASSWORD = "yoursmtppassword" # contains usually 16 characters

RECEIVER_EMAIL = "who.your.sending-it-to@domain.com"

SUBJECT = "[Newsletter] Number 13"
MESSAGE = "Hello, Please find the pdf below, yours faithfully"

TOWNS = ["paris", "lyon"]
```

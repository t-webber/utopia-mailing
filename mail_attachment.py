""" Send an email with attachment """

from email import encoders
from email.mime.text import MIMEText
from email.mime.base import MIMEBase
from email.mime.multipart import MIMEMultipart
import ssl
import smtplib
import glob
from credentials import SENDER_EMAIL, RECEIVER_EMAIL, SUBJECT, MESSAGE, SMTP_SERVER, PORT, PASSWORD, TOWNS

MONTPELLIER = True
AVIGNON = False


# Create a multipart message and set headers
message = MIMEMultipart()
message["From"] = SENDER_EMAIL
message["To"] = RECEIVER_EMAIL
message["Subject"] = SUBJECT

# Add body to email
message.attach(MIMEText(MESSAGE, "plain"))


def get_filename(city):
    """ Gets the latest pdf for the given city """
    files = glob.glob(f"{city}_*.pdf")
    max_idx = 0
    max_file = ""
    for file in files:
        curr = int(file.split("_")[1].split(".")[0])
        if max_idx < curr:
            max_idx = curr
            max_file = file
    return max_file


def get_part_pdf(city):
    """ Creates a part to attach to message for a pdf """

    filename = get_filename(city)

    # Open file in bynary mode
    with open(filename, "rb") as attachment:
        # Add file as application/octet-stream
        # Email client can usually download this automatically as attachment
        part = MIMEBase("application", "octet-stream")
        part.set_payload(attachment.read())

    # Encode file in ASCII characters to send by email
    encoders.encode_base64(part)

    # Add header as pdf attachment
    part.add_header(
        "Content-Disposition",
        f"attachment; filename= {filename}",
    )

    return part


for town in TOWNS:
    town_part = get_part_pdf(town)
    message.attach(town_part)

# Add attachment to message and convert message to string
text = message.as_string()

# Log in to server using secure context and send email
context = ssl.create_default_context()
with smtplib.SMTP(SMTP_SERVER, PORT) as server:
    server.starttls(context=context)
    server.login(SENDER_EMAIL, PASSWORD)
    server.sendmail(SENDER_EMAIL, RECEIVER_EMAIL, text)

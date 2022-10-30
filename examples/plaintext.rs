use std::net::TcpStream;

fn main() {
    // REMINDER this is unsafe, the credentials are sent over the connection in CLEARTEXT
    // Anyone or anything between this connection and the server can read your login creds!
    // Please oh please do not use this where this is even a possibility.
    match plaintext() {
        Ok(conn) => {
            eprintln!("Connection successful!");
            println!("{:?}",conn);
        },
        Err(e) => {
            eprintln!("Connection error!");
            eprintln!("{:?}",e);
        }
    }
}

fn plaintext() -> imap::error::Result<Option<String>> {
    // Make a raw TCP connection to an UNSAFE IMAP server
    let stream = TcpStream::connect("imap.example.com:143").unwrap(); // This is unsafe.
    let mut client = imap::Client::new(stream);
    client.read_greeting()?;
    eprintln!("\nUNENCRYPTED connection made!!!!\n");
    eprintln!("This is highly not recommended.\n");
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client.login("user", "pass").unwrap();

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let mut body;
    match message.body() {
        Some(msg) => {
            body = std::str::from_utf8(msg)
                .expect("message was not valid utf-8")
                .to_string();
        }
        None => body = "".to_string(),
    }
    // be nice to the server and log out
    imap_session.logout()?;
    Ok(Some(body))
}
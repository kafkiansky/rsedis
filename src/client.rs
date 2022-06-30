use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use crate::Command;
use std::fmt;

pub struct Conf {
    pub host: String,
    pub password: Option<String>,
    pub database: Option<u8>,
}

pub struct Client {
    conn: TcpStream
}

pub struct RedisError {
    err: String
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Error occurred: {}", &self.err)
    }
}

impl Client {
    pub fn connect(conf: Conf) -> Result<Client, RedisError> {
        match TcpStream::connect(conf.host) {
            Ok(mut stream) => {
                if let Some(pass) = conf.password {
                    if let Err(e) = stream.write(Command::auth(pass).as_bytes()) {
                        return Err(RedisError{err: e.to_string()})
                    }
                }

                if let Some(db) = conf.database {
                    if let Err(e) = stream.write(Command::select_database(db).as_bytes()) {
                        return Err(RedisError{err: e.to_string()})
                    }
                }

                Ok(Client{conn: stream})
            },
            Err(e) => Err(RedisError{err: e.to_string()})
        }
    }

    pub fn command(&mut self, c: Command) {
        self.conn.write(c.as_bytes()).expect("TODO: panic message");
    }
}

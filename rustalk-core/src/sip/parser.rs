//! SIP Message Parser using nom

use super::{Header, Message, Method, Request, Response, StatusCode, Uri};
use bytes::Bytes;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map, map_res, opt},
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

/// Parse a complete SIP message
pub fn parse_message(input: &[u8]) -> Result<Message, String> {
    let input_str = std::str::from_utf8(input).map_err(|e| e.to_string())?;

    // Try to parse as request first
    if let Ok((_, request)) = parse_request(input_str) {
        return Ok(Message::Request(request));
    }

    // Try to parse as response
    if let Ok((_, response)) = parse_response(input_str) {
        return Ok(Message::Response(response));
    }

    Err("Invalid SIP message".to_string())
}

fn parse_request(input: &str) -> IResult<&str, Request> {
    let (input, (method, _, uri, _, version, _)) = tuple((
        parse_method,
        space1,
        parse_uri,
        space1,
        tag("SIP/2.0"),
        line_ending,
    ))(input)?;

    let (input, headers) = many0(parse_header)(input)?;
    let (input, _) = line_ending(input)?;

    let body = Bytes::from(input.as_bytes().to_vec());

    Ok((
        "",
        Request {
            method,
            uri,
            version: version.to_string(),
            headers,
            body,
        },
    ))
}

fn parse_response(input: &str) -> IResult<&str, Response> {
    let (input, (version, _, status, _, reason, _)) = tuple((
        tag("SIP/2.0"),
        space1,
        map_res(digit1, |s: &str| s.parse::<u16>()),
        space1,
        take_until("\r\n"),
        line_ending,
    ))(input)?;

    let (input, headers) = many0(parse_header)(input)?;
    let (input, _) = line_ending(input)?;

    let body = Bytes::from(input.as_bytes().to_vec());

    Ok((
        "",
        Response {
            version: version.to_string(),
            status_code: StatusCode(status),
            reason_phrase: reason.to_string(),
            headers,
            body,
        },
    ))
}

fn parse_method(input: &str) -> IResult<&str, Method> {
    alt((
        map(tag("INVITE"), |_| Method::Invite),
        map(tag("ACK"), |_| Method::Ack),
        map(tag("BYE"), |_| Method::Bye),
        map(tag("CANCEL"), |_| Method::Cancel),
        map(tag("REGISTER"), |_| Method::Register),
        map(tag("OPTIONS"), |_| Method::Options),
        map(tag("INFO"), |_| Method::Info),
        map(tag("PRACK"), |_| Method::Prack),
        map(tag("SUBSCRIBE"), |_| Method::Subscribe),
        map(tag("NOTIFY"), |_| Method::Notify),
        map(tag("UPDATE"), |_| Method::Update),
        map(tag("REFER"), |_| Method::Refer),
        map(tag("MESSAGE"), |_| Method::Message),
    ))(input)
}

fn parse_uri(input: &str) -> IResult<&str, Uri> {
    let (input, scheme) = alt((tag("sip"), tag("sips")))(input)?;
    let (input, _) = char(':')(input)?;

    // Try to parse user part
    let (input, user) = opt(terminated(
        take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_' || c == '.'),
        char('@'),
    ))(input)?;

    // Parse host
    let (input, host) = take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '-')(input)?;

    // Parse optional port
    let (input, port) = opt(preceded(
        char(':'),
        map_res(digit1, |s: &str| s.parse::<u16>()),
    ))(input)?;

    Ok((
        input,
        Uri {
            scheme: scheme.to_string(),
            user: user.map(|u| u.to_string()),
            host: host.to_string(),
            port,
            params: Vec::new(),
        },
    ))
}

fn parse_header(input: &str) -> IResult<&str, Header> {
    let (input, (name, value)) = terminated(
        separated_pair(
            take_while1(|c: char| c.is_alphanumeric() || c == '-'),
            tuple((char(':'), opt(space1))),
            take_until("\r\n"),
        ),
        line_ending,
    )(input)?;

    Ok((
        input,
        Header::new(name.to_string(), value.trim().to_string()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options_request() {
        let msg = b"OPTIONS sip:example.com SIP/2.0\r\n\
                     Via: SIP/2.0/UDP client.example.com:5060\r\n\
                     From: <sip:alice@example.com>\r\n\
                     To: <sip:bob@example.com>\r\n\
                     Call-ID: abc123\r\n\
                     CSeq: 1 OPTIONS\r\n\
                     Content-Length: 0\r\n\
                     \r\n";

        let result = parse_message(msg);
        assert!(result.is_ok());

        let message = result.unwrap();
        assert!(message.is_request());

        let request = message.as_request().unwrap();
        assert_eq!(request.method, Method::Options);
        assert_eq!(request.uri.host, "example.com");
    }

    #[test]
    fn test_parse_200_response() {
        let msg = b"SIP/2.0 200 OK\r\n\
                     Via: SIP/2.0/UDP server.example.com:5060\r\n\
                     From: <sip:alice@example.com>\r\n\
                     To: <sip:bob@example.com>\r\n\
                     Call-ID: abc123\r\n\
                     CSeq: 1 OPTIONS\r\n\
                     Content-Length: 0\r\n\
                     \r\n";

        let result = parse_message(msg);
        assert!(result.is_ok());

        let message = result.unwrap();
        assert!(message.is_response());

        let response = message.as_response().unwrap();
        assert_eq!(response.status_code, StatusCode::OK);
    }
}

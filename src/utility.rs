pub fn request(url: &str, authorization: Option<String>, data: String) {
    let mut client = reqwest::blocking::Client::new()
        .patch(url)
        .body(data.clone())
        .header(reqwest::header::CONTENT_TYPE, "application/json");

    if let Some(token) = authorization {
        client = client.header(
            reqwest::header::AUTHORIZATION,
            std::env::var("TOKEN").unwrap_or_else(|_| match token.is_empty() {
                true => panic!("error: a discord token was not specified!"),
                false => token.clone(),
            }),
        );
    }

    client.send().expect("error: unable to send request!");
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

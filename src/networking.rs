extern crate rustc_serialize;

use rustc_serialize::json;
use hyper::header::Headers;
use hyper::client::Client;
use std::io::Read;

pub fn post_request(url: &str, client: &Client, headers: Headers, body_string: &str) {

    #[derive(RustcEncodable)]
    struct CommentPost<'a> {
        pub body: &'a str,
    }

    let post = CommentPost { body: body_string };

    if let Ok(json) = json::encode(&post) {
        let response = client.post(url)
            .headers(headers.clone())
            .body(&json)
            .send();
        println!("{:?}", response);
    } else {
        println!("Failed to encode comment post.");
    }
}

pub fn network_request(url: &str, client: &Client, headers: Headers) -> Option<(String, Headers)> {
	if let Ok(mut response) = client.get(url)
        .headers(headers.clone())
        .send() {
        let mut response_buffer = String::new();

        response.read_to_string(&mut response_buffer);
        let response_headers = response.headers.clone();
        return Some((response_buffer, response_headers));
    }
    return None;
}

pub fn get_model_from_network<T: rustc_serialize::Decodable>(url: &str,
                                                      client: &Client,
                                                      headers: Headers) -> Option<T> {
      if let Some(response_buffer) = network_request(url, &client, headers) {
      		let request: Result<T, rustc_serialize::json::DecoderError> =
            json::decode(&response_buffer.0);

      		return request.ok();
  	  }
      return None;
}

pub fn get_models_from_network<T: rustc_serialize::Decodable>(url: &str,
                                                      client: &Client,
                                                      headers: Headers)
                                                      -> Vec<T> {


        // println!("{:?}", response_buffer);
      if let Some(response_buffer) = network_request(url, &client, headers.clone()) {
       	 let requests: Result<Vec<T>, rustc_serialize::json::DecoderError> =
            json::decode(&response_buffer.0);

       	 if let Ok(requests) = requests {
      	  	 if let Some(next_url) = next_url(&response_buffer.1) {
          	  let mut mutable_requests = requests;
         	   mutable_requests.extend(get_models_from_network(&next_url, client, headers));
         	   return mutable_requests;
       	 };
       	 	return requests;
    	};	
	}
    return Vec::new();
}

pub fn next_url(headers: &Headers) -> Option<String> {
    if let Some(link_header) = headers.get_raw("Link") {
        let ref first = link_header[0];

        if let Ok(whole_link_header) = String::from_utf8(first.clone()) {

            // println!("{:?}", whole_link_header);

            let parts: Vec<&str> = whole_link_header.split(",")
                .filter(|x| x.contains("rel=\"next\""))
                .collect();

            if !parts.is_empty() {
                let next = parts[0].to_string();

                if let Some(indices) = next.find("<")
                    .and_then(|x| next.find(">").and_then(|y| Some((x, y)))) {
                    let url = &next[indices.0 + 1..indices.1];
                    println!("{:?}", url);
                    return Some(url.to_string());
                }

                return None;
            };
        };

    };

    return None;
}

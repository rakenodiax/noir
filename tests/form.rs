#[macro_use] extern crate json;
#[macro_use] extern crate noir;
#[macro_use]
mod base_test;
test!();


// Form Uploads ---------------------------------------------------------------
#[test]
fn test_with_form_body_url_encoded() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "field" => "someValue",
                "array[]" => vec![1, 2, 3, 4, 5]
            })
            .expected_body("field=someValue&array%5B%5D=1&array%5B%5D=2&array%5B%5D=3&array%5B%5D=4&array%5B%5D=5")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_url_encoded_trailing_comma() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "field" => "someValue",
                "array[]" => vec![1, 2, 3, 4, 5],
            })
            .expected_body("field=someValue&array%5B%5D=1&array%5B%5D=2&array%5B%5D=3&array%5B%5D=4&array%5B%5D=5")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_multipart_vec_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "vec_file" => (
                    "file.bin",
                    Mime(TopLevel::Application, SubLevel::OctetStream, vec![]),
                    vec![1, 2, 3, 4, 5, 6, 7, 8]
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"vec_file\"; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_multipart_str_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "str_file" => (
                    "readme.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    "Hello World"
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"str_file\"; filename=\"readme.md\"\r\nContent-Type: text/plain\r\n\r\nHello World\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_multipart_string_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "string_file" => (
                    "readme.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    "Hello World".to_string()
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"string_file\"; filename=\"readme.md\"\r\nContent-Type: text/plain\r\n\r\nHello World\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_multipart_json_file() {

    let actual = {
        API::post("/form")
            .with_body(form! {
                "json_file" => (
                    "data.json",
                    Mime(TopLevel::Application, SubLevel::Json, vec![]),
                    object! {
                        "key" => "value"
                    }
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"json_file\"; filename=\"data.json\"\r\nContent-Type: application/json\r\n\r\n{\"key\":\"value\"}\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

#[test]
fn test_with_form_body_multipart_fs_file() {

    use std::fs::File;

    let actual = {
        API::post("/form")
            .with_body(form! {
                "fs_file" => (
                    "form_test.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    File::open("./tests/form_test.md").unwrap()
                )
            })
            .expected_body("\r\n--<boundary>\r\nContent-Disposition: form-data; name=\"fs_file\"; filename=\"form_test.md\"\r\nContent-Type: text/plain\r\n\r\nForm Test Data File\n\r\n--<boundary>--\r\n")
            .collect()
    };

    assert_pass!(actual);

}

// Form Parsing Errors --------------------------------------------------------
#[test]
fn test_with_form_body_error_missing_disposition_header() {

    let actual = {
        API::post("/response/forward")
            .with_header(ContentType(
                Mime(TopLevel::Application, SubLevel::FormData, vec![
                     (Attr::Boundary, Value::Ext("boundary".to_string()))
                ]))
            )
            .with_body("\r\n--boundary\r\nContent-Type: application/octet-stream\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--boundary--\r\n")
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "field" => "value"
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>form body could not be parsed:

              <br>Content-Disposition header is missing from multi part field.


"#, actual);

}

#[test]
fn test_with_form_body_error_broken_headers() {

    let actual = {
        API::post("/response/forward")
            .with_header(ContentType(
                Mime(TopLevel::Application, SubLevel::FormData, vec![
                     (Attr::Boundary, Value::Ext("boundary".to_string()))
                ]))
            )
            .with_body("\r\n--boundary\r\nContent-Type\n application/octet-stream\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--boundary--\r\n")
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "field" => "value"
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>form body could not be parsed:

              <br>Invalid byte in header name of multi part field.


"#, actual);

}

#[test]
fn test_with_form_body_error_filename_invalid_utf8() {

    let actual = {
        API::post("/response/forward")
            .with_header(ContentType(
                Mime(TopLevel::Application, SubLevel::FormData, vec![
                     (Attr::Boundary, Value::Ext("boundary".to_string()))
                ]))
            )
            .with_body("\r\n--boundary\r\nContent-Disposition: form-data; name=\"fs_file\"; filename=\"form_\u{0}\u{1}test.md\"\r\nContent-Type: application/octet-stream\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--boundary--\r\n")
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "field" => "value"
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>form body could not be parsed:

              <br>Invalid byte in header value of multi part field.


"#, actual);

}

#[test]
fn test_with_form_body_error_too_many_headers() {

    let actual = {
        API::post("/response/forward")
            .with_header(ContentType(
                Mime(TopLevel::Application, SubLevel::FormData, vec![
                     (Attr::Boundary, Value::Ext("boundary".to_string()))
                ]))
            )
            .with_body("\r\n--boundary\r\nContent-Disposition: form-data; name=\"fs_file\"; filename=\"form_test.md\"\r\nContent-Type: application/octet-stream\r\nX-Superfluous-Header: Foo\r\n\r\n\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\r\n--boundary--\r\n")
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "field" => "value"
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>form body could not be parsed:

              <br>Unexpected headers in multi part field.


"#, actual);

}


// Form File Body Data --------------------------------------------------------
#[test]
fn test_with_form_body_multipart_file_raw_mismatch() {

    let actual = {
        API::post("/response/forward")
            .with_body(form! {
                "vec_file" => (
                    "file.bin",
                    Mime(TopLevel::Application, SubLevel::OctetStream, vec![]),
                    vec![1, 2, 3, 4, 5, 6, 7, 8]
                )
            })
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "vec_file" => (
                        "file.bin",
                        Mime(TopLevel::Application, SubLevel::OctetStream, vec![]),
                        vec![1, 2, 3]
                    )
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>body form data does not match:

              - <bb>form.<bb>vec_file: <by>File<by> <by>raw body data does not match, expected the following <bg>3 bytes<by>:

                   [<bg>0x01, <bg>0x02, <bg>0x03]

                <by>but got the following <br>8 bytes <by>instead:

                   [<br>0x01, <br>0x02, <br>0x03, <br>0x04, <br>0x05, <br>0x06, <br>0x07, <br>0x08]


"#, actual);

}

#[test]
fn test_with_form_body_multipart_file_text_mismatch() {

    let actual = {
        API::post("/response/forward")
            .with_body(form! {
                "str_file" => (
                    "readme.md",
                    Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                    "Hello World"
                )
            })
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "str_file" => (
                        "readme.md",
                        Mime(TopLevel::Text, SubLevel::Plain, vec![]),
                        "World"
                    )
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>body form data does not match:

              - <bb>form.<bb>str_file: <by>File<by> <by>text body does not match, expected:

                    \"<bg>World\"

                <by>but got:

                    \"<br>Hello World\"

                <by>difference:

                    \"<gbg>Hello World\"


"#, actual);

}

#[test]
fn test_with_form_body_multipart_file_json_mismatch() {

    let actual = {
        API::post("/response/forward")
            .with_body(form! {
                "json_file" => (
                    "data.json",
                    Mime(TopLevel::Application, SubLevel::Json, vec![]),
                    object! {
                        "key" => "value",
                        "additional" => "key"
                    }
                )
            })
            .provide(responses![
                EXAMPLE.post("/forward").expected_body(form! {
                    "json_file" => (
                        "data.json",
                        Mime(TopLevel::Application, SubLevel::Json, vec![]),
                        object! {
                            "key" => "valueTwo"
                        }
                    )
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>body form data does not match:

              - <bb>form.<bb>json_file: <by>File<by> <by>body JSON does not match:

                    - <bb>json.<bb>key: <bg>String <by>does not match, expected:

                          \"<bg>value\"

                      <by>but got:

                          \"<br>valueTwo\"

                      <by>difference:

                          \"<gbr>value <gbg>valueTwo\"


"#, actual);

}

#[test]
fn test_with_form_body_multipart_file_json_mismatch_exact() {

    let actual = {
        API::post("/response/forward")
            .with_body(form! {
                "json_file" => (
                    "data.json",
                    Mime(TopLevel::Application, SubLevel::Json, vec![]),
                    object! {
                        "key" => "value",
                        "additional" => "key"
                    }
                )
            })
            .provide(responses![
                EXAMPLE.post("/forward").expected_exact_body(form! {
                    "json_file" => (
                        "data.json",
                        Mime(TopLevel::Application, SubLevel::Json, vec![]),
                        object! {
                            "key" => "valueTwo"
                        }
                    )
                })
            ])
            .collect()
    };

    assert_fail!(r#"
<br>Response Failure: <bn>POST <by>request to \"<bn>http://localhost:4000<bn>/response/forward\" <by>returned <br>1 <by>error(s)

<bb> 1) <br>Request Failure: <bn>POST <by>response provided for \"<bn>https://example.com<bn>/forward\" <by>returned <br>1 <by>error(s)

    <bb> 1.1) <by>Request <by>body form data does not match:

              - <bb>form.<bb>json_file: <by>File<by> <by>body JSON does not match:

                    - <bb>json.<bb>key: <bg>String <by>does not match, expected:

                          \"<bg>value\"

                      <by>but got:

                          \"<br>valueTwo\"

                      <by>difference:

                          \"<gbr>value <gbg>valueTwo\"

                    - <bb>json: <bg>Object <by>has <br>1 <by>additional unexpected key(s) (<br>additional)


"#, actual);

}


//! HTTP-boundary tests for `MusicBrainzClient` and `CoverArtClient`. Use
//! `wiremock` to stand in for the real services so CI never hits the
//! network.

use super::client::MusicBrainzClient;
use super::cover_art::CoverArtClient;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TEST_UA: &str = "amaterasu-player/test (no-contact)";

#[tokio::test]
async fn search_release_group_sends_user_agent_and_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/release-group"))
        .and(header("user-agent", TEST_UA))
        .and(query_param("fmt", "json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(SAMPLE_RG_RESPONSE))
        .mount(&server)
        .await;

    let client = MusicBrainzClient::new(TEST_UA, server.uri()).expect("client builds");
    let resp = client
        .search_release_group(Some("Pink Floyd"), "Dark Side of the Moon", 5)
        .await
        .expect("ok");
    assert_eq!(resp.release_groups.len(), 1);
    assert_eq!(resp.release_groups[0].title, "Dark Side of the Moon");
    assert_eq!(resp.release_groups[0].score, 100);
}

#[tokio::test]
async fn retries_on_503_then_succeeds() {
    let server = MockServer::start().await;
    // First call: 503. Second call: 200.
    Mock::given(method("GET"))
        .and(path("/release-group"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/release-group"))
        .respond_with(ResponseTemplate::new(200).set_body_string(SAMPLE_RG_RESPONSE))
        .mount(&server)
        .await;

    let client = MusicBrainzClient::new(TEST_UA, server.uri()).expect("client builds");
    let resp = client
        .search_release_group(None, "Dark Side of the Moon", 5)
        .await
        .expect("retry succeeds");
    assert_eq!(resp.release_groups.len(), 1);
}

#[tokio::test]
async fn caa_returns_none_on_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/release/00000000-0000-0000-0000-000000000000/front-500",
        ))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = CoverArtClient::new(TEST_UA, server.uri()).expect("client builds");
    let result = client
        .fetch_front("00000000-0000-0000-0000-000000000000")
        .await
        .expect("404 is Ok(None), not Err");
    assert!(result.is_none());
}

#[tokio::test]
async fn caa_returns_bytes_on_200() {
    let server = MockServer::start().await;
    let body = vec![1, 2, 3, 4, 5];
    Mock::given(method("GET"))
        .and(path("/release/abcd-1234/front-500"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(body.clone()))
        .mount(&server)
        .await;

    let client = CoverArtClient::new(TEST_UA, server.uri()).expect("client builds");
    let result = client.fetch_front("abcd-1234").await.expect("ok");
    assert_eq!(result, Some(body));
}

const SAMPLE_RG_RESPONSE: &str = r#"{
  "release-groups": [
    {
      "id": "f5093c06-23e3-404f-aeaa-40f72885ee3a",
      "title": "Dark Side of the Moon",
      "score": 100,
      "primary-type": "Album",
      "first-release-date": "1973-03-01",
      "artist-credit": [
        {
          "name": "Pink Floyd",
          "artist": {
            "id": "83d91898-7763-47d7-b03b-b92132375c47",
            "name": "Pink Floyd",
            "sort-name": "Pink Floyd"
          }
        }
      ],
      "releases": [
        {
          "id": "0d8d2354-d7c3-4cf3-90fb-7a45c2eb09c1",
          "title": "The Dark Side of the Moon",
          "date": "1973-03-01",
          "country": "GB"
        }
      ]
    }
  ]
}"#;

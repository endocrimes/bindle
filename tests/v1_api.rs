//! Tests for the v1 API endpoints. These are integration style tests and, as such, they run through
//! entire user workflows

mod common;

#[tokio::test]
async fn test_successful_workflow() {
    let bindles = common::load_all_files().await;
    let (store, index) = common::setup();

    let api = bindle::server::routes::api(store, index);

    // Upload the parcels for one of the invoices
    let valid_v1 = bindles.get("valid_v1").expect("Missing scaffold");

    for k in valid_v1.label_files.keys() {
        let res = valid_v1
            .parcel_body(k)
            .method("POST")
            .path("/v1/_p/")
            .reply(&api)
            .await;
        assert_eq!(
            res.status(),
            warp::http::StatusCode::OK,
            "Body: {}",
            String::from_utf8_lossy(res.body())
        );
        // Make sure the label we get back is valid toml
        toml::from_slice::<bindle::Label>(res.body()).expect("should be valid label TOML");
    }

    // Create an invoice pointing to those parcels and make sure the correct response is returned
    let res = warp::test::request()
        .method("POST")
        .header("Content-Type", "application/toml")
        .path("/v1/_i")
        .body(&valid_v1.invoice)
        .reply(&api)
        .await;

    assert_eq!(
        res.status(),
        warp::http::StatusCode::CREATED,
        "Body: {}",
        String::from_utf8_lossy(res.body())
    );
    let create_res: bindle::InvoiceCreateResponse =
        toml::from_slice(res.body()).expect("should be valid invoice response TOML");

    assert!(
        create_res.missing.is_none(),
        "Invoice should not have missing parcels"
    );

    // Create a second version of the same invoice with missing parcels and make sure the correct response is returned
    let valid_v2 = bindles.get("valid_v2").expect("Missing scaffold");

    let res = warp::test::request()
        .method("POST")
        .header("Content-Type", "application/toml")
        .path("/v1/_i")
        .body(&valid_v2.invoice)
        .reply(&api)
        .await;

    assert_eq!(
        res.status(),
        warp::http::StatusCode::ACCEPTED,
        "Body: {}",
        String::from_utf8_lossy(res.body())
    );
    let create_res: bindle::InvoiceCreateResponse =
        toml::from_slice(res.body()).expect("should be valid invoice response TOML");

    assert_eq!(
        create_res
            .missing
            .expect("Should have missing parcels")
            .len(),
        1,
        "Invoice should not have missing parcels"
    );

    // Get an invoice
    let res = warp::test::request()
        .path("/v1/_i/enterprise.com/warpcore/1.0.0")
        .reply(&api)
        .await;
    assert_eq!(
        res.status(),
        warp::http::StatusCode::OK,
        "Body: {}",
        String::from_utf8_lossy(res.body())
    );
    let inv: bindle::Invoice = toml::from_slice(res.body()).expect("should be valid invoice TOML");

    // Get a parcel
    let parcel = &inv.parcels.expect("Should have parcels")[0];
    let res = warp::test::request()
        .path(&format!("/v1/_p/{}", parcel.label.sha256))
        .reply(&api)
        .await;
    assert_eq!(
        res.status(),
        warp::http::StatusCode::OK,
        "Body: {}",
        String::from_utf8_lossy(res.body())
    );

    assert_eq!(
        res.body().as_ref(),
        valid_v1.parcel_files.get("parcel").unwrap().as_slice()
    );
    assert_eq!(
        res.headers()
            .get("Content-Type")
            .expect("No content type header found")
            .to_str()
            .unwrap(),
        parcel.label.media_type
    );
}

#[tokio::test]
async fn test_yank() {
    // Upload the parcels for one of the invoices

    // Yank the invoice

    // Attempt to fetch the invoice and make sure it doesn't return

    // Set yanked to true and attempt to fetch again
}

#[tokio::test]
// This isn't meant to test all of the possible validation failures (that should be done in a unit
// test for storage), just the main validation failures from the API
async fn test_invoice_validation() {
    // Already created invoice

    // Missing version
}

#[tokio::test]
// This isn't meant to test all of the possible validation failures (that should be done in a unit
// test for storage), just the main validation failures from the API
async fn test_parcel_validation() {
    // Already created parcel

    // Incorrect SHA

    // Missing size

    // Empty body?
}

#[tokio::test]
// Once again, this isn't meant to exercise all of the query functionality, just that the API
// functions properly
async fn test_queries() {
    // Insert data into store

    // Test empty query

    // Test query term filter

    // Test version queries

    // Test yank

    // Test limit/offset
}

// https://datatracker.ietf.org/doc/html/rfc8030#section-4
// POST /subscribe HTTP/1.1
// Host: push.example.net

//    HTTP/1.1 201 Created
// Date: Thu, 11 Dec 2014 23:56:52 GMT
// Link: </push/JzLQ3raZJfFBR0aqvOMsLrt54w4rJUsV>;
//         rel="urn:ietf:params:push"
// Link: </subscription-set/4UXwi2Rd7jGS7gp5cuutF8ZldnEuvbOy>;
//         rel="urn:ietf:params:push:set"
// Location: https://push.example.net/subscription/LBhhw0OohO-Wl4Oi971UG

// https://datatracker.ietf.org/doc/html/rfc8030#section-5
// POST /push/JzLQ3raZJfFBR0aqvOMsLrt54w4rJUsV HTTP/1.1
// Host: push.example.net
// TTL: 15
// Content-Type: text/plain;charset=utf8
// Content-Length: 36
//
// iChYuI3jMzt3ir20P8r_jgRR-dSuN182x7iB

// HTTP/1.1 201 Created
// Date: Thu, 11 Dec 2014 23:56:55 GMT
// Location: https://push.example.net/message/qDIYHNcfAIPP_5ITvURr-d6BGt

use std::sync::Arc;

use axum::Router;

use crate::WebState;

pub fn router(state: Arc<WebState>) -> Router<Arc<WebState>> {
    Router::new().with_state(state)
    // TODO: add API routes
}

use crate::errors::failed_request_execution;

use super::Request;

pub async fn check<'a>(request: &Request<'a>) -> bool {
    let response = request
        .client
        .get(&format!("http://{}/health_check", request.socket_addr()))
        .send()
        .await
        .expect(failed_request_execution());
    response.status().is_success()
}

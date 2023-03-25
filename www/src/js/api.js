import { getCognitoIdToken } from "@/js/cognito.js";

/*
 * Retry function: Allow the request (via fn function) to be done up to 3 times
 * before giving up...
 */
function retry(fn, retries=2) {
  return fn()
  .then((r) => {
    if (! r.ok && retries > 0) { // Response is an error & can retries
      // console.log("Retrying - number of retries left: ", retries);
      return retry(fn, retries - 1);
    } else {
      return r;
    }
  });
}

export function get(user, url) {
  return getCognitoIdToken(user)
  .then((idToken) => retry(() => fetch(url, {
        headers: {
          Authorization: idToken,
        }
      }))
  )
  .then(handle_response);
}

export function post(user, url, data) {
  return getCognitoIdToken(user)
    .then((idToken) => retry(() => fetch(url, {
        method: 'POST',
        headers: { 
          Authorization: idToken, 
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      }))
    )
    .then(handle_response);
}

export function put(user, url, data) {
  return getCognitoIdToken(user)
    .then((idToken) => retry(() => fetch(url, {
        method: 'PUT',
        headers: { 
          Authorization: idToken, 
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      }))
    )
    .then(handle_response);
}

export function patch(user, url, data) {
  return getCognitoIdToken(user)
    .then((idToken) => retry(() => fetch(url, {
        method: 'PATCH',
        headers: { 
          Authorization: idToken, 
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
      }))
    )
    .then(handle_response);
}

function handle_response(r) {
  if (! r.ok) {throw new Error(`Query failed with status code ${r.statusText}`);}
  return r.json();
}


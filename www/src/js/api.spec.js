import { get, post, put, patch } from './api.js';

import { getCognitoIdToken } from "@/js/cognito.js";
jest.mock("@/js/cognito.js");

describe('js/api.js', () => {
  it('handles get requests', async () => {
    global.fetch = jest.fn((url) => Promise.resolve({ ok: true, json: () => ({type: "response", url}), }) );
    getCognitoIdToken.mockImplementation(() => { return Promise.resolve("idToken123"); });
    let r = await get({user: "user"}, "/test");
    expect(global.fetch.mock.calls.length).toEqual(1);
    expect(global.fetch.mock.calls[0][0]).toEqual("/test");
    expect(global.fetch.mock.calls[0][1].headers.Authorization).toEqual("idToken123");
  });

  it('retries 3 time on response not ok (internal server error for ex) and throws an error', async () => {
    global.fetch = jest.fn((url) => Promise.resolve({ ok: false }));
    getCognitoIdToken.mockImplementation(() => { return Promise.resolve("idToken123"); });
    await expect(get({user: "user"}, "/test") ).rejects.toThrowError(/failed/);
    expect(global.fetch.mock.calls.length).toEqual(3);
    expect(global.fetch.mock.calls[2][0]).toEqual("/test");
    expect(global.fetch.mock.calls[2][1].headers.Authorization).toEqual("idToken123");
  });

  it('performs post requests', async () => {
    global.fetch = jest.fn((url) => Promise.resolve({ok: true, json: () => ({})}));
    getCognitoIdToken.mockImplementation(() => { return Promise.resolve("idToken123"); });
    await post({user: "user"}, "/post", {type: "Post"});
    expect(global.fetch.mock.calls[0][0]).toEqual("/post");
    expect(global.fetch.mock.calls[0][1].method).toEqual('POST');
    expect(JSON.parse(global.fetch.mock.calls[0][1].body).type).toEqual("Post");
  });

  it('performs put requests', async () => {
    global.fetch = jest.fn((url) => Promise.resolve({ok: true, json: () => ({})}));
    getCognitoIdToken.mockImplementation(() => { return Promise.resolve("idToken123"); });
    await put({user: "user"}, "/put", {type: "Put"});
    expect(global.fetch.mock.calls[0][0]).toEqual("/put");
    expect(global.fetch.mock.calls[0][1].method).toEqual('PUT');
    expect(JSON.parse(global.fetch.mock.calls[0][1].body).type).toEqual("Put");
  });

  it('performs patch requests', async () => {
    global.fetch = jest.fn((url) => Promise.resolve({ok: true, json: () => ({})}));
    getCognitoIdToken.mockImplementation(() => { return Promise.resolve("idToken123"); });
    await patch({user: "user"}, "/patch", {type: "Patch"});
    expect(global.fetch.mock.calls[0][0]).toEqual("/patch");
    expect(global.fetch.mock.calls[0][1].method).toEqual('PATCH');
    expect(JSON.parse(global.fetch.mock.calls[0][1].body).type).toEqual("Patch");
  });

  it('reject an error if cognito fails', async () => {
    global.fetch = jest.fn((url) => Promise.reject({ok: true, json: () => ({})}));
    getCognitoIdToken.mockImplementation(() => { return Promise.reject("CognitoFailure"); });
    await expect(patch({user: "user"}, "/patch", {type: "Patch"})).rejects.toEqual("CognitoFailure");
    expect(global.fetch.mock.calls.length).toEqual(0);
  });
});

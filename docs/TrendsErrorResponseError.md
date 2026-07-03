# TrendsErrorResponseError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **Code** | Machine-readable error code. Possible values:   * `BAD_REQUEST` — malformed request, invalid path/query parameter, or JSON type mismatch in the body. Returned with HTTP 400.   * `VALIDATION_ERROR` — one or more request fields failed validation rules; the message names each offending field. Returned with HTTP 422 (Unprocessable Entity).   * `AMBIGUOUS_LOOKUP` — a slug identifier matched more than one record; qualify it (e.g. `country/league`) and retry.   * `UNAUTHORIZED` — missing or invalid credentials.   * `FORBIDDEN` — authenticated but not permitted to access this resource.   * `NOT_FOUND` — resource does not exist.   * `RATE_LIMIT_EXCEEDED` — too many requests.   * `INTERNAL_ERROR` — unexpected server error.  (enum: BAD_REQUEST, VALIDATION_ERROR, AMBIGUOUS_LOOKUP, UNAUTHORIZED, FORBIDDEN, NOT_FOUND, RATE_LIMIT_EXCEEDED, INTERNAL_ERROR) | 
**message** | **String** | Human-readable explanation of the error. For `VALIDATION_ERROR` responses, names the offending field(s) and the rule that failed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



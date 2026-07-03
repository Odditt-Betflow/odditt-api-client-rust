# \OddsApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_odds_upcoming_odds_by_event_post**](OddsApi.md#v1_odds_upcoming_odds_by_event_post) | **POST** /v1/odds/upcoming-odds-by-event | Get upcoming odds by event



## v1_odds_upcoming_odds_by_event_post

> serde_json::Value v1_odds_upcoming_odds_by_event_post(v1_odds_upcoming_odds_by_event_post_request)
Get upcoming odds by event

Returns paginated betting market positions for a specific upcoming event, optionally filtered by operator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_odds_upcoming_odds_by_event_post_request** | [**V1OddsUpcomingOddsByEventPostRequest**](V1OddsUpcomingOddsByEventPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


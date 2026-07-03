# \EventsApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_events_upcoming_events_post**](EventsApi.md#v1_events_upcoming_events_post) | **POST** /v1/events/upcoming-events | Get upcoming events



## v1_events_upcoming_events_post

> serde_json::Value v1_events_upcoming_events_post(v1_events_upcoming_events_post_request)
Get upcoming events

Returns upcoming events with optional sport and league filters, timezone support, and pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_events_upcoming_events_post_request** | [**V1EventsUpcomingEventsPostRequest**](V1EventsUpcomingEventsPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \TelemetryApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_events_post**](TelemetryApi.md#v1_affiliates_events_post) | **POST** /v1/affiliates/events | Log an impression or click



## v1_affiliates_events_post

> v1_affiliates_events_post(v1_affiliates_events_post_request)
Log an impression or click

Fire-and-forget telemetry. Records one impression (card render) or click (deeplink tap). Returns 202 on accept; never blocks the widget's click-out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_events_post_request** | [**V1AffiliatesEventsPostRequest**](V1AffiliatesEventsPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


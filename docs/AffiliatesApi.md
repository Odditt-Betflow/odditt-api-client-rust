# \AffiliatesApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_event_position_post**](AffiliatesApi.md#v1_affiliates_event_position_post) | **POST** /v1/affiliates/event-position | Single-bet cart for affiliate mode
[**v1_affiliates_parlay_post**](AffiliatesApi.md#v1_affiliates_parlay_post) | **POST** /v1/affiliates/parlay | Parlay cart for affiliate mode



## v1_affiliates_event_position_post

> serde_json::Value v1_affiliates_event_position_post(v1_affiliates_event_position_post_request)
Single-bet cart for affiliate mode

Returns live odds across operators for a single event betting market position, overlaid with the calling client's best affiliate offer per operator. Powers the widget's \"Bet Now\" cart for a single selection. Results are scoped to the authenticated client; operators can be gated with operator_ids / operator_keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_event_position_post_request** | [**V1AffiliatesEventPositionPostRequest**](V1AffiliatesEventPositionPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_parlay_post

> serde_json::Value v1_affiliates_parlay_post(v1_affiliates_parlay_post_request)
Parlay cart for affiliate mode

Returns combined parlay odds per operator, the per-leg single bets, and the calling client's best affiliate offer per operator, with combo deeplinks generated per operator. Powers the widget's \"Bet Now\" cart for a multi-leg bet. Results are scoped to the authenticated client; operators can be gated with operator_ids / operator_keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_parlay_post_request** | [**V1AffiliatesParlayPostRequest**](V1AffiliatesParlayPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


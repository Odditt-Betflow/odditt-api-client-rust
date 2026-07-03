# \OffersApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_deals_post**](OffersApi.md#v1_affiliates_deals_post) | **POST** /v1/affiliates/deals | Paginated client deals
[**v1_affiliates_offers_post**](OffersApi.md#v1_affiliates_offers_post) | **POST** /v1/affiliates/offers | Client offer cards for a geography



## v1_affiliates_deals_post

> serde_json::Value v1_affiliates_deals_post(v1_affiliates_deals_post_request)
Paginated client deals

Paginated client deals (carousel/featured/list), optionally enriched with operator reviews. Scoped to the authenticated client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_deals_post_request** | Option<[**V1AffiliatesDealsPostRequest**](V1AffiliatesDealsPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_offers_post

> serde_json::Value v1_affiliates_offers_post(v1_affiliates_offers_post_request)
Client offer cards for a geography

Returns the calling client's best operator offer card per operator for a geography. The core offers read the widget cart renders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_offers_post_request** | Option<[**V1AffiliatesOffersPostRequest**](V1AffiliatesOffersPostRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


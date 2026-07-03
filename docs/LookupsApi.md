# \LookupsApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_lookups_affiliate_networks_get**](LookupsApi.md#v1_affiliates_lookups_affiliate_networks_get) | **GET** /v1/affiliates/lookups/affiliate-networks | Affiliate networks
[**v1_affiliates_lookups_offer_categories_get**](LookupsApi.md#v1_affiliates_lookups_offer_categories_get) | **GET** /v1/affiliates/lookups/offer-categories | Offer categories
[**v1_affiliates_lookups_operator_jurisdictions_get**](LookupsApi.md#v1_affiliates_lookups_operator_jurisdictions_get) | **GET** /v1/affiliates/lookups/operator-jurisdictions | Operator jurisdictions
[**v1_affiliates_lookups_operators_get**](LookupsApi.md#v1_affiliates_lookups_operators_get) | **GET** /v1/affiliates/lookups/operators | Operators



## v1_affiliates_lookups_affiliate_networks_get

> serde_json::Value v1_affiliates_lookups_affiliate_networks_get()
Affiliate networks

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_lookups_offer_categories_get

> serde_json::Value v1_affiliates_lookups_offer_categories_get()
Offer categories

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_lookups_operator_jurisdictions_get

> serde_json::Value v1_affiliates_lookups_operator_jurisdictions_get(operator_id, country_code)
Operator jurisdictions

Where we believe an operator is live. Informational only — not an upload gate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operator_id** | Option<**i64**> |  |  |
**country_code** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_lookups_operators_get

> serde_json::Value v1_affiliates_lookups_operators_get(search, page, page_size)
Operators

Paginated list of operators, optionally filtered by a free-text search term.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 20]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


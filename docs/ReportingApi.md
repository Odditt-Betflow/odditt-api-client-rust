# \ReportingApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_links_clicks_get**](ReportingApi.md#v1_affiliates_links_clicks_get) | **GET** /v1/affiliates/links/clicks | Impression/click rollups
[**v1_affiliates_links_inventory_summary_get**](ReportingApi.md#v1_affiliates_links_inventory_summary_get) | **GET** /v1/affiliates/links/inventory-summary | Inventory counts



## v1_affiliates_links_clicks_get

> serde_json::Value v1_affiliates_links_clicks_get(start_date, end_date, operator_id, country_code, subnational_region_code, offer_campaign, offer_label)
Impression/click rollups

Per-link, per-day impression and click counts for the authenticated client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **chrono::NaiveDate** |  | [required] |
**end_date** | **chrono::NaiveDate** |  | [required] |
**operator_id** | Option<**i64**> |  |  |
**country_code** | Option<**String**> |  |  |
**subnational_region_code** | Option<**String**> |  |  |
**offer_campaign** | Option<**String**> |  |  |
**offer_label** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_links_inventory_summary_get

> serde_json::Value v1_affiliates_links_inventory_summary_get(group_by)
Inventory counts

Active/inactive link counts grouped by the requested dimension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_by** | Option<**String**> |  |  |[default to operator]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


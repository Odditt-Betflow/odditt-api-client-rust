# \HistoricalApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_historical_events_get**](HistoricalApi.md#v1_historical_events_get) | **GET** /v1/historical/events | List historical odds events (paginated)
[**v1_historical_odds_event_id_ext_get**](HistoricalApi.md#v1_historical_odds_event_id_ext_get) | **GET** /v1/historical/odds/{event_id}.{ext} | Download a per-event historical odds file



## v1_historical_events_get

> serde_json::Value v1_historical_events_get(schema_version, sport_id, league_id, team_id, start_date, end_date, page, page_size)
List historical odds events (paginated)

Returns a paginated catalog of per-event historical odds files the authenticated client has access to. Each result entry carries pre-baked download URLs for the JSON and CSV payloads; clients GET those URLs directly. Results are scoped to the authenticated client; callers without a client association receive `403 Forbidden`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_version** | Option<**String**> | Payload schema version. Defaults to `v1` when omitted. Unknown versions are rejected. |  |[default to v1]
**sport_id** | Option<**i64**> |  |  |
**league_id** | Option<**i64**> |  |  |
**team_id** | Option<**i64**> | Filter to events involving this team as either home or away. |  |
**start_date** | Option<**String**> | Inclusive lower bound on the event/affiliation date, ISO format YYYY-MM-DD. |  |
**end_date** | Option<**String**> | Inclusive upper bound on the event/affiliation date, ISO format YYYY-MM-DD. |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_historical_odds_event_id_ext_get

> serde_json::Value v1_historical_odds_event_id_ext_get(event_id, ext, schema_version)
Download a per-event historical odds file

Resolves the requested per-event file for the authenticated client and streams the JSON or CSV payload back. The response body is the raw file content (not the manifest envelope); the API takes care of fetching the bytes from the underlying CDN so callers see a single HTTP call per file.  The `sha256` of the bytes-at-rest is exposed as a strong `ETag`. Clients can short-circuit with `If-None-Match` to receive `304 Not Modified`. `Range` requests are forwarded upstream and `206 Partial Content` responses are relayed unchanged — useful for resuming multi-MB CSV downloads. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **i64** | Canonical event identifier. | [required] |
**ext** | **String** | File format. `json` returns `application/json`; `csv` returns `text/csv`. | [required] |
**schema_version** | Option<**String**> | Payload schema version. Defaults to `v1` when omitted. |  |[default to v1]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


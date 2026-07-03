# \TrendsApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_trends_by_betting_market_position_post**](TrendsApi.md#v1_trends_by_betting_market_position_post) | **POST** /v1/trends/by-betting-market-position | Get flows by betting market position IDs
[**v1_trends_flow_graph_data_fact_flow_id_get**](TrendsApi.md#v1_trends_flow_graph_data_fact_flow_id_get) | **GET** /v1/trends/flow-graph-data/{fact_flow_id} | Get fact flow graph data
[**v1_trends_flow_tooltip_flow_type_flow_id_get**](TrendsApi.md#v1_trends_flow_tooltip_flow_type_flow_id_get) | **GET** /v1/trends/flow-tooltip/{flow_type}/{flow_id} | Get flow tooltip payload
[**v1_trends_flows_by_id_post**](TrendsApi.md#v1_trends_flows_by_id_post) | **POST** /v1/trends/flows-by-id | Get flows by ids
[**v1_trends_flows_post**](TrendsApi.md#v1_trends_flows_post) | **POST** /v1/trends/flows | Get flows (paginated)
[**v1_trends_flows_quote_post**](TrendsApi.md#v1_trends_flows_quote_post) | **POST** /v1/trends/flows/quote | Quote a batch of flows against the caller's configured check endpoint
[**v1_trends_flows_quote_sandbox_post**](TrendsApi.md#v1_trends_flows_quote_sandbox_post) | **POST** /v1/trends/flows/quote-sandbox | Mock operator pricing endpoint — for sandbox / development use
[**v1_trends_leagues_with_available_flows_get**](TrendsApi.md#v1_trends_leagues_with_available_flows_get) | **GET** /v1/trends/leagues-with-available-flows | Get leagues with available flows
[**v1_trends_mixed_flows_post**](TrendsApi.md#v1_trends_mixed_flows_post) | **POST** /v1/trends/mixed-flows | Get mixed flows (paginated)
[**v1_trends_widget_event_post**](TrendsApi.md#v1_trends_widget_event_post) | **POST** /v1/trends/widget/event | Submit a widget telemetry event



## v1_trends_by_betting_market_position_post

> serde_json::Value v1_trends_by_betting_market_position_post(v1_trends_by_betting_market_position_post_request)
Get flows by betting market position IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_trends_by_betting_market_position_post_request** | [**V1TrendsByBettingMarketPositionPostRequest**](V1TrendsByBettingMarketPositionPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flow_graph_data_fact_flow_id_get

> serde_json::Value v1_trends_flow_graph_data_fact_flow_id_get(fact_flow_id)
Get fact flow graph data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fact_flow_id** | **i64** | The fact flow ID to get graph data for | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flow_tooltip_flow_type_flow_id_get

> serde_json::Value v1_trends_flow_tooltip_flow_type_flow_id_get(flow_type, flow_id)
Get flow tooltip payload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_type** | **String** | Type of flow (fact or fun) | [required] |
**flow_id** | **i64** | The flow ID to get tooltip data for | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flows_by_id_post

> serde_json::Value v1_trends_flows_by_id_post(v1_trends_flows_by_id_post_request)
Get flows by ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_trends_flows_by_id_post_request** | [**V1TrendsFlowsByIdPostRequest**](V1TrendsFlowsByIdPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flows_post

> serde_json::Value v1_trends_flows_post(v1_trends_flows_post_request)
Get flows (paginated)

Same parameters as mixed-flows but returns non-mixed (single-type) flows. Uses the same underlying request model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_trends_flows_post_request** | [**V1TrendsFlowsPostRequest**](V1TrendsFlowsPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flows_quote_post

> Vec<models::TrendsQuoteItemResult> v1_trends_flows_quote_post(trends_quote_flow_item)
Quote a batch of flows against the caller's configured check endpoint

Accepts a JSON array of 1..50 flow specifications and, for each one, forwards a hydrated payload to the URL the caller has registered under `/v1/account/check-endpoint`. Outbound calls run in parallel; the response is a JSON array with one entry per input item, in input order. Per-item failures are encoded in each entry's `status` (and `body`) so one bad operator response does not poison the others. Batch-level failures map to HTTP 4xx without an array body — 412 when the caller has not configured a check endpoint, 422 on validation failure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trends_quote_flow_item** | [**Vec<models::TrendsQuoteFlowItem>**](TrendsQuoteFlowItem.md) |  | [required] |

### Return type

[**Vec<models::TrendsQuoteItemResult>**](trends_QuoteItemResult.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_flows_quote_sandbox_post

> models::TrendsQuoteSandboxResponse v1_trends_flows_quote_sandbox_post(trends_quote_sandbox_request)
Mock operator pricing endpoint — for sandbox / development use

Drop-in stand-in for a real operator's pricing endpoint. Accepts ONE hydrated pricing payload (the exact shape that /v1/trends/flows/quote POSTs to a configured check endpoint) and returns synthetic pricing- response data with deterministic `button_payload` values and randomly- injected failure modes (~15% rate). Integrators wire this URL into /v1/account/check-endpoint as `url` to round-trip the widget call entirely on platform infrastructure without a real operator. Not for production traffic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trends_quote_sandbox_request** | [**TrendsQuoteSandboxRequest**](TrendsQuoteSandboxRequest.md) |  | [required] |

### Return type

[**models::TrendsQuoteSandboxResponse**](trends_QuoteSandboxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_leagues_with_available_flows_get

> serde_json::Value v1_trends_leagues_with_available_flows_get(sport_id, sport_key)
Get leagues with available flows

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sport_id** | Option<**i64**> |  |  |
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Format: {sport_key}. Alternative to sport_id. If both are provided, sport_id takes precedence. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_mixed_flows_post

> serde_json::Value v1_trends_mixed_flows_post(v1_trends_flows_post_request)
Get mixed flows (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_trends_flows_post_request** | [**V1TrendsFlowsPostRequest**](V1TrendsFlowsPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_trends_widget_event_post

> models::TrendsWidgetEventResponse v1_trends_widget_event_post(trends_widget_event_request)
Submit a widget telemetry event

Records a single client-side widget interaction (impression, click, dwell, or cart action) for analytics. Accepts a typed envelope `{event_type, mode, event_body}`: `event_type` is a closed enum naming the interaction, optional `mode` describes the widget mode, and `event_body` is an arbitrary JSON object whose shape depends on the event. Available only to widget API keys. Fire-and-forget: returns `202 Accepted` once the event is queued; the response does not guarantee durable storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trends_widget_event_request** | [**TrendsWidgetEventRequest**](TrendsWidgetEventRequest.md) |  | [required] |

### Return type

[**models::TrendsWidgetEventResponse**](trends_WidgetEventResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


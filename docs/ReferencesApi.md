# \ReferencesApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_references_betting_market_categories_get**](ReferencesApi.md#v1_references_betting_market_categories_get) | **GET** /v1/references/betting-market-categories | Get betting market categories
[**v1_references_betting_market_positions_get**](ReferencesApi.md#v1_references_betting_market_positions_get) | **GET** /v1/references/betting-market-positions | Get all betting market positions
[**v1_references_betting_markets_get**](ReferencesApi.md#v1_references_betting_markets_get) | **GET** /v1/references/betting-markets | Get betting markets (paginated)
[**v1_references_countries_get**](ReferencesApi.md#v1_references_countries_get) | **GET** /v1/references/countries | Get countries (paginated)
[**v1_references_event_periods_get**](ReferencesApi.md#v1_references_event_periods_get) | **GET** /v1/references/event-periods | Get event periods
[**v1_references_leagues_get**](ReferencesApi.md#v1_references_leagues_get) | **GET** /v1/references/leagues | Get leagues (paginated)
[**v1_references_odds_format_get**](ReferencesApi.md#v1_references_odds_format_get) | **GET** /v1/references/odds-format | Get odds formats (paginated)
[**v1_references_operators_get**](ReferencesApi.md#v1_references_operators_get) | **GET** /v1/references/operators | Get operators (paginated)
[**v1_references_players_get**](ReferencesApi.md#v1_references_players_get) | **GET** /v1/references/players | Get players (paginated)
[**v1_references_sports_get**](ReferencesApi.md#v1_references_sports_get) | **GET** /v1/references/sports | Get sports (paginated)
[**v1_references_subnational_regions_get**](ReferencesApi.md#v1_references_subnational_regions_get) | **GET** /v1/references/subnational-regions | Get subnational regions (paginated)
[**v1_references_tag_dimensions_get**](ReferencesApi.md#v1_references_tag_dimensions_get) | **GET** /v1/references/tag-dimensions | Get tag dimensions
[**v1_references_tag_types_search_get**](ReferencesApi.md#v1_references_tag_types_search_get) | **GET** /v1/references/tag-types/search | Search tag types
[**v1_references_tag_types_tag_type_id_children_get**](ReferencesApi.md#v1_references_tag_types_tag_type_id_children_get) | **GET** /v1/references/tag-types/{tag_type_id}/children | Get child tag types
[**v1_references_teams_get**](ReferencesApi.md#v1_references_teams_get) | **GET** /v1/references/teams | Get teams (paginated)



## v1_references_betting_market_categories_get

> serde_json::Value v1_references_betting_market_categories_get(sport_id, sport_key)
Get betting market categories

Returns available betting market categories, optionally filtered by sport.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sport_id** | Option<**i64**> |  |  |
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Alternative to sport_id. If both are provided, sport_id takes precedence. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_betting_market_positions_get

> serde_json::Value v1_references_betting_market_positions_get()
Get all betting market positions

Returns all available betting market positions.

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


## v1_references_betting_markets_get

> serde_json::Value v1_references_betting_markets_get(sport_id, sport_key, search, page, page_size)
Get betting markets (paginated)

Returns a paginated list of betting markets with optional sport filter and text search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sport_id** | Option<**i64**> |  |  |
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Alternative to sport_id. If both are provided, sport_id takes precedence. |  |
**search** | Option<**String**> | Text search filter for betting market names |  |
**page** | Option<**i64**> | Page number for pagination |  |[default to 1]
**page_size** | Option<**i64**> | Number of results per page |  |[default to 100]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_countries_get

> models::TrendsPaginatedResponse v1_references_countries_get(search, page, page_size)
Get countries (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search by country name |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_event_periods_get

> serde_json::Value v1_references_event_periods_get(sport_id, sport_key)
Get event periods

Returns available event periods, optionally filtered by sport.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sport_id** | Option<**i64**> |  |  |
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Alternative to sport_id. If both are provided, sport_id takes precedence. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_leagues_get

> serde_json::Value v1_references_leagues_get(country_id, sport_id, sport_key, search, is_popular, page, page_size, search_mode)
Get leagues (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_id** | Option<**i64**> |  |  |
**sport_id** | Option<**i64**> |  |  |
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Format: {sport_key}. Alternative to sport_id. If both are provided, sport_id takes precedence. |  |
**search** | Option<**String**> |  |  |
**is_popular** | Option<**bool**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]
**search_mode** | Option<**String**> | Search mode (e.g. exact match vs partial). When omitted, uses default search behavior. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_odds_format_get

> models::TrendsPaginatedResponse v1_references_odds_format_get(search, page, page_size)
Get odds formats (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_operators_get

> models::TrendsPaginatedResponse v1_references_operators_get(search, page, page_size)
Get operators (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search by operator display name |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_players_get

> serde_json::Value v1_references_players_get(team_id, team_key, search, page, page_size, search_mode)
Get players (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**i64**> |  |  |
**team_key** | Option<**String**> | Team external key (e.g. 'new-england-patriots', 'nfl.new-england-patriots'). Format: {team_key} or {league_key}.{team_key}. Alternative to team_id. If both are provided, team_id takes precedence. |  |
**search** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]
**search_mode** | Option<**String**> | Search mode (e.g. exact match vs partial). When omitted, uses default search behavior. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_sports_get

> models::TrendsPaginatedResponse v1_references_sports_get(search, page, page_size)
Get sports (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_subnational_regions_get

> models::TrendsPaginatedResponse v1_references_subnational_regions_get(country_id, search, page, page_size)
Get subnational regions (paginated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_id** | **i64** | Country ID to filter subnational regions | [required] |
**search** | Option<**String**> | Search by subnational region name |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_tag_dimensions_get

> models::V1ReferencesTagDimensionsGet200Response v1_references_tag_dimensions_get(flow_type)
Get tag dimensions

Returns the top-level tag dimension categories. Use these as entry points to explore the tag hierarchy. Optionally filter to dimensions relevant to a specific flow type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_type** | Option<**String**> | Filter by flow type eligibility. |  |

### Return type

[**models::V1ReferencesTagDimensionsGet200Response**](_v1_references_tag_dimensions_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_tag_types_search_get

> models::TrendsPaginatedResponse v1_references_tag_types_search_get(search, search_mode, dimension, flow_type, terminal_only, tag_level, page, page_size)
Search tag types

Search across tag types by keyword. Returns paginated results with breadcrumb paths for disambiguation. Use terminal_only combined with flow_type to find only tags that are usable as filters for a specific flow type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
**search_mode** | Option<**String**> | How the search term is matched. Defaults to 'starts_with'. |  |
**dimension** | Option<**String**> | Limit results to a specific dimension (e.g. 'event', 'metadata'). |  |
**flow_type** | Option<**String**> | Filter by flow type eligibility. |  |
**terminal_only** | Option<**bool**> | When true, returns only terminal (filterable) tag types. |  |[default to false]
**tag_level** | Option<**i64**> | Restrict results to a specific hierarchy depth. |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]

### Return type

[**models::TrendsPaginatedResponse**](trends_PaginatedResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_tag_types_tag_type_id_children_get

> models::V1ReferencesTagTypesTagTypeIdChildrenGet200Response v1_references_tag_types_tag_type_id_children_get(tag_type_id, include_values, flow_type)
Get child tag types

Returns the direct children of a given tag type, along with context about the parent (description and known values). Use this to navigate the tag hierarchy from dimensions down to individual filterable tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_type_id** | **i64** | Parent tag type ID to drill into. | [required] |
**include_values** | Option<**bool**> | When true, includes individual value-level leaf tags in the results. When false (default), value-level leaves are omitted — the parent's known_values field already enumerates them. |  |[default to false]
**flow_type** | Option<**String**> | Filter by flow type eligibility. |  |

### Return type

[**models::V1ReferencesTagTypesTagTypeIdChildrenGet200Response**](_v1_references_tag_types__tag_type_id__children_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_references_teams_get

> serde_json::Value v1_references_teams_get(league_id, league_key, search, page, page_size, search_mode, start_date, end_date)
Get teams (paginated)

Returns a paginated list of teams. When neither `start_date` nor `end_date` is provided, only teams whose affiliation is currently active are returned. When either date is provided, the result is filtered to teams whose affiliation overlapped the requested window — useful for looking up teams that played in a league during a past season. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**league_id** | Option<**i64**> |  |  |
**league_key** | Option<**String**> | League external key (e.g. 'nba', 'united-states.nba'). Format: {league_key} or {country_key}.{league_key}. Alternative to league_id. If both are provided, league_id takes precedence. |  |
**search** | Option<**String**> |  |  |
**page** | Option<**i64**> |  |  |[default to 1]
**page_size** | Option<**i64**> |  |  |[default to 20]
**search_mode** | Option<**String**> | Search mode (e.g. exact match vs partial). When omitted, uses default search behavior. |  |
**start_date** | Option<**String**> | Inclusive lower bound on the event/affiliation date, ISO format YYYY-MM-DD. |  |
**end_date** | Option<**String**> | Inclusive upper bound on the event/affiliation date, ISO format YYYY-MM-DD. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


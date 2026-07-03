# \LinksApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_affiliates_links_bulk_deactivate_post**](LinksApi.md#v1_affiliates_links_bulk_deactivate_post) | **POST** /v1/affiliates/links/bulk-deactivate | Bulk deactivate links by filter
[**v1_affiliates_links_bulk_patch**](LinksApi.md#v1_affiliates_links_bulk_patch) | **PATCH** /v1/affiliates/links/bulk | Bulk patch links by filter
[**v1_affiliates_links_bulk_post**](LinksApi.md#v1_affiliates_links_bulk_post) | **POST** /v1/affiliates/links/bulk | Bulk create/upsert links (JSON or CSV)
[**v1_affiliates_links_jobs_get**](LinksApi.md#v1_affiliates_links_jobs_get) | **GET** /v1/affiliates/links/jobs | List async bulk jobs
[**v1_affiliates_links_jobs_job_id_get**](LinksApi.md#v1_affiliates_links_jobs_job_id_get) | **GET** /v1/affiliates/links/jobs/{job_id} | Poll an async bulk job
[**v1_affiliates_links_post**](LinksApi.md#v1_affiliates_links_post) | **POST** /v1/affiliates/links | Create or upsert a single link



## v1_affiliates_links_bulk_deactivate_post

> serde_json::Value v1_affiliates_links_bulk_deactivate_post(v1_affiliates_links_bulk_deactivate_post_request)
Bulk deactivate links by filter

Soft-deletes all links matching a filter (e.g. a state goes offline).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_links_bulk_deactivate_post_request** | [**V1AffiliatesLinksBulkDeactivatePostRequest**](V1AffiliatesLinksBulkDeactivatePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_links_bulk_patch

> serde_json::Value v1_affiliates_links_bulk_patch(v1_affiliates_links_bulk_patch_request)
Bulk patch links by filter

Applies a patch to all links matching a filter (e.g. rewrite all FanDuel-NJ URLs on a domain change).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_links_bulk_patch_request** | [**V1AffiliatesLinksBulkPatchRequest**](V1AffiliatesLinksBulkPatchRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_links_bulk_post

> serde_json::Value v1_affiliates_links_bulk_post(request_body, dry_run, r#async)
Bulk create/upsert links (JSON or CSV)

Upserts many links idempotently on the natural key. Accepts a JSON array of row objects or a text/csv body (header row). Per-row errors never fail the batch. Use ?dry_run=true to validate without persisting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) |  | [required] |
**dry_run** | Option<**bool**> | When true, validates the whole batch and returns the per-row report without persisting. Always synchronous. |  |
**r#async** | Option<**bool**> | Force background processing. Batches of 500+ rows are queued automatically regardless of this flag. Queued batches return 202 with a job_id to poll at /v1/affiliates/links/jobs/{job_id}. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_links_jobs_get

> serde_json::Value v1_affiliates_links_jobs_get()
List async bulk jobs

Recent async bulk jobs for the authenticated client.

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


## v1_affiliates_links_jobs_job_id_get

> serde_json::Value v1_affiliates_links_jobs_job_id_get(job_id)
Poll an async bulk job

Status, counts, and per-row errors for one async bulk job. Poll until status is \"completed\" or \"failed\". Scoped to the authenticated client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_affiliates_links_post

> serde_json::Value v1_affiliates_links_post(v1_affiliates_links_post_request)
Create or upsert a single link

Creates a link, or updates the existing one on the natural key (operator + geography + category + campaign + label). Resolves operator, country, and region at write time; unresolvable codes return 422.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**v1_affiliates_links_post_request** | [**V1AffiliatesLinksPostRequest**](V1AffiliatesLinksPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


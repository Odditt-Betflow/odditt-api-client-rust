# \AccountApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_account_api_keys_get**](AccountApi.md#v1_account_api_keys_get) | **GET** /v1/account/api-keys | List own API keys
[**v1_account_api_keys_key_code_delete**](AccountApi.md#v1_account_api_keys_key_code_delete) | **DELETE** /v1/account/api-keys/{key_code} | Deactivate an API key
[**v1_account_api_keys_post**](AccountApi.md#v1_account_api_keys_post) | **POST** /v1/account/api-keys | Create a new API key
[**v1_account_check_endpoint_delete**](AccountApi.md#v1_account_check_endpoint_delete) | **DELETE** /v1/account/check-endpoint | Delete the check endpoint
[**v1_account_check_endpoint_get**](AccountApi.md#v1_account_check_endpoint_get) | **GET** /v1/account/check-endpoint | Get the configured check endpoint
[**v1_account_check_endpoint_post**](AccountApi.md#v1_account_check_endpoint_post) | **POST** /v1/account/check-endpoint | Set the check endpoint (upsert)
[**v1_account_config_get**](AccountApi.md#v1_account_config_get) | **GET** /v1/account/config | Get own client configuration
[**v1_account_secret_post**](AccountApi.md#v1_account_secret_post) | **POST** /v1/account/secret | Create a new client secret
[**v1_account_secrets_get**](AccountApi.md#v1_account_secrets_get) | **GET** /v1/account/secrets | List own client secrets
[**v1_account_secrets_secret_code_delete**](AccountApi.md#v1_account_secrets_secret_code_delete) | **DELETE** /v1/account/secrets/{secret_code} | Delete a client secret
[**v1_account_usage_get**](AccountApi.md#v1_account_usage_get) | **GET** /v1/account/usage | Get own usage stats



## v1_account_api_keys_get

> models::AuthListApiKeysResponse v1_account_api_keys_get()
List own API keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthListApiKeysResponse**](auth_ListAPIKeysResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_api_keys_key_code_delete

> v1_account_api_keys_key_code_delete(key_code)
Deactivate an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_code** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_api_keys_post

> models::AuthCreateApiKeyResponse v1_account_api_keys_post(auth_create_api_key_request)
Create a new API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_create_api_key_request** | [**AuthCreateApiKeyRequest**](AuthCreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::AuthCreateApiKeyResponse**](auth_CreateAPIKeyResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_check_endpoint_delete

> v1_account_check_endpoint_delete()
Delete the check endpoint

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_check_endpoint_get

> models::AuthCheckEndpointResponse v1_account_check_endpoint_get()
Get the configured check endpoint

Returns the URL, method and bearer token preview for the check endpoint used by flow quoting. The full bearer token is never returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthCheckEndpointResponse**](auth_CheckEndpointResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_check_endpoint_post

> models::AuthCheckEndpointResponse v1_account_check_endpoint_post(auth_set_check_endpoint_request)
Set the check endpoint (upsert)

Stores or updates the URL, method and bearer token used to quote flows against the client's own service. A single configuration exists per client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_set_check_endpoint_request** | [**AuthSetCheckEndpointRequest**](AuthSetCheckEndpointRequest.md) |  | [required] |

### Return type

[**models::AuthCheckEndpointResponse**](auth_CheckEndpointResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_config_get

> v1_account_config_get()
Get own client configuration

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_secret_post

> models::AuthCreateSecretResponse v1_account_secret_post()
Create a new client secret

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthCreateSecretResponse**](auth_CreateSecretResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_secrets_get

> models::AuthListSecretsResponse v1_account_secrets_get()
List own client secrets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthListSecretsResponse**](auth_ListSecretsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_secrets_secret_code_delete

> v1_account_secrets_secret_code_delete(secret_code)
Delete a client secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_code** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_account_usage_get

> v1_account_usage_get(start_date, end_date)
Get own usage stats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


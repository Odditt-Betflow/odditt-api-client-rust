# \AuthenticationApi

All URIs are relative to *https://api.odditt.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_auth_login_post**](AuthenticationApi.md#v1_auth_login_post) | **POST** /v1/auth/login | Login with API key
[**v1_auth_refresh_post**](AuthenticationApi.md#v1_auth_refresh_post) | **POST** /v1/auth/refresh | Refresh tokens
[**v1_oauth_login_post**](AuthenticationApi.md#v1_oauth_login_post) | **POST** /v1/oauth/login | OAuth login (client credentials)
[**v1_oauth_refresh_post**](AuthenticationApi.md#v1_oauth_refresh_post) | **POST** /v1/oauth/refresh | OAuth refresh



## v1_auth_login_post

> models::AuthTokenPair v1_auth_login_post(x_api_key)
Login with API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |

### Return type

[**models::AuthTokenPair**](auth_TokenPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_auth_refresh_post

> models::AuthTokenPair v1_auth_refresh_post(auth_refresh_request)
Refresh tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_refresh_request** | [**AuthRefreshRequest**](AuthRefreshRequest.md) |  | [required] |

### Return type

[**models::AuthTokenPair**](auth_TokenPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_oauth_login_post

> models::AuthTokenPair v1_oauth_login_post(auth_o_auth_login_request)
OAuth login (client credentials)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_o_auth_login_request** | [**AuthOAuthLoginRequest**](AuthOAuthLoginRequest.md) |  | [required] |

### Return type

[**models::AuthTokenPair**](auth_TokenPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_oauth_refresh_post

> models::AuthTokenPair v1_oauth_refresh_post(auth_refresh_request)
OAuth refresh

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_refresh_request** | [**AuthRefreshRequest**](AuthRefreshRequest.md) |  | [required] |

### Return type

[**models::AuthTokenPair**](auth_TokenPair.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


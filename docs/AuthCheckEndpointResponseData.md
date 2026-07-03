# AuthCheckEndpointResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_multiple** | Option<**bool**> |  | [optional]
**auth_header_name** | Option<**String**> |  | [optional]
**auth_scheme** | Option<**String**> | May be empty for raw-value header schemes like 'X-API-Key'. | [optional]
**auth_token_preview** | Option<**String**> | Masked preview of the stored auth token (e.g. \"…aB3x\"). The full token is never returned. | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**method** | Option<**Method**> |  (enum: POST) | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# TrendsQuoteItemResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> | Operator response passthrough on 200, error envelope otherwise. | 
**flow_id** | **i64** |  | 
**flow_type** | **FlowType** |  (enum: fact_flow, fun_flow, fact_flow_parlay, fun_flow_parlay, plain_flow_parlay) | 
**status** | **Status** | HTTP-style status code for this single item. 200 = success, 404 = flow not resolvable, 502 = upstream returned non-2xx or non-JSON, 504 = upstream timed out. (enum: 200, 404, 500, 502, 504) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



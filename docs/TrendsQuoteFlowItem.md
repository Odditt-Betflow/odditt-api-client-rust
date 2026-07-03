# TrendsQuoteFlowItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country** | **String** | ISO 3166-1 alpha-2 country code (e.g. 'US', 'IT'). Uppercased server-side. | 
**event_betting_market_position_ids** | **Vec<i64>** | Leg identifiers that make up this flow. Length 1 = single, length 2+ = parlay. | 
**flow_id** | **i64** |  | 
**flow_type** | **FlowType** |  (enum: fact_flow, fun_flow, fact_flow_parlay, fun_flow_parlay, plain_flow_parlay) | 
**region** | Option<**String**> | Sub-national region code (e.g. 'NJ', 'WA'). Uppercased server-side. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



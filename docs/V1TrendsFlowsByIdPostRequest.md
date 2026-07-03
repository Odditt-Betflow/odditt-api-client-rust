# V1TrendsFlowsByIdPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fact_flow_type** | Option<**FactFlowType**> | Sub-type for fact flows. Defaults to 'base' if omitted. (enum: base, expanded) | [optional]
**flow_ids** | **Vec<i64>** | Array of flow IDs to retrieve | 
**flow_type** | **FlowType** |  (enum: fact_flow, fun_flow, fact_flow_parlay, fun_flow_parlay, plain_flow_parlay) | 
**use_cartoon_images** | Option<**bool**> | When true, the logo fields on each flow, leg, and multi-trend slot (default_logo_url, logo_url_1, logo_url_2) are replaced with cartoon-jersey image URLs derived from the relevant team, player, or league. When false or omitted, the original logo URLs are returned. Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



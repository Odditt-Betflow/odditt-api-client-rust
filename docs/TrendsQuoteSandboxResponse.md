# TrendsQuoteSandboxResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**american_odds_value** | Option<**String**> | Parlay only. Null when uncombinable. | [optional]
**button_payload** | Option<**String**> | Synthetic bet-slip identifier (prefix 'FD-MOCK-'). Null when the result is invalid or the parlay is uncombinable. | [optional]
**decimal_odds_value** | Option<**f64**> |  | [optional]
**event_betting_market_position_maps** | **Vec<serde_json::Value>** |  | 
**flow_id** | **i64** |  | 
**flow_type** | **String** |  | 
**fractional_odds_value** | Option<**String**> |  | [optional]
**is_combinable** | Option<**bool**> | Parlay only. False if any leg failed or the parlay-level uncombinable roll fired. | [optional]
**odds_implied_probability** | Option<**f64**> |  | [optional]
**parlay_failure_reason** | Option<**ParlayFailureReason**> |  (enum: one or more legs failed, selections are not combinable, ) | [optional]
**payout_multiplier** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



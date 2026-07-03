# V1TrendsByBettingMarketPositionPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_betting_market_position_ids** | **Vec<i64>** | Array of event betting market position IDs | 
**odds_format** | Option<**OddsFormat**> | Odds display format. Defaults per product_mode (dfs→multiplier, prediction_market→percent, else american). (enum: american, decimal, fractional, multiplier, percent) | [optional]
**product_mode** | Option<**ProductMode**> | Display mode. dfs rewrites stat lines to MORE/LESS; on the paginated flows endpoints it also auto-filters to over/under player props (entity_type=player, positions [4,5]) unless an entity/position filter is set. (enum: sportsbook, dfs, prediction_market) | [optional]
**use_cartoon_images** | Option<**bool**> | When true, the logo fields on each flow, leg, and multi-trend slot (default_logo_url, logo_url_1, logo_url_2) are replaced with cartoon-jersey image URLs derived from the relevant team, player, or league. When false or omitted, the original logo URLs are returned. Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



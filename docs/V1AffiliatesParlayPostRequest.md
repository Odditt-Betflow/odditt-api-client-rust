# V1AffiliatesParlayPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 country code. | [optional][default to US]
**event_betting_market_position_ids** | **Vec<i64>** | The event betting market positions that make up the parlay legs. | 
**hide_offers_links** | Option<**bool**> | When true, offer metadata is returned without the deep-link URLs. | [optional]
**odds_format** | Option<**OddsFormat**> | Odds display format. Defaults per product_mode (dfs→multiplier, prediction_market→percent, else american). (enum: american, decimal, fractional, multiplier, percent) | [optional]
**offer_campaign** | Option<**String**> | Optional campaign filter. Renders only offers tagged with this campaign. | [optional]
**operator_ids** | Option<**Vec<i64>**> | Optional list of operator IDs to gate which operators appear in the cart. | [optional]
**operator_keys** | Option<**Vec<String>**> | Optional operator external keys (e.g. 'draftkings'). Resolved IDs are merged with operator_ids. | [optional]
**product_mode** | Option<**ProductMode**> | Display mode. dfs rewrites leg stat lines to MORE/LESS. (enum: sportsbook, dfs, prediction_market) | [optional]
**subnational_region_code** | Option<**String**> | ISO 3166-2 subnational region code (used for offer/deeplink resolution). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



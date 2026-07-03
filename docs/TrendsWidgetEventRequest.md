# TrendsWidgetEventRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_body** | **std::collections::HashMap<String, serde_json::Value>** | Arbitrary JSON object carrying event-specific fields. May be an empty object. | 
**event_type** | **EventType** | The kind of client-side widget interaction being reported. (enum: impression, click, dwell, cart_open, cart_offers_shown, cart_dismiss, card_click, graph_expand, graph_metric_change, filter_change, widget-impression, widget-viewed, betslip-clicked, betslip-shared, flow-clicked, flow-shared) | 
**mode** | Option<**Mode**> | Optional widget mode the interaction occurred in. Defaults to `clean`. (enum: operator, affiliate, clean) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



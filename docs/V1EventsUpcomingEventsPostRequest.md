# V1EventsUpcomingEventsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_date** | Option<**chrono::NaiveDate**> | Filter by event date (YYYY-MM-DD). Defaults to today (UTC). | [optional]
**league_id** | Option<**i64**> |  | [optional]
**league_key** | Option<**String**> | League external key (e.g. 'nba', 'united-states.nba'). Alternative to league_id. If both are provided, league_id takes precedence. | [optional]
**page** | Option<**i64**> | Page number for pagination | [optional][default to 1]
**page_size** | Option<**i64**> | Number of events per page | [optional][default to 100]
**sport_id** | Option<**i64**> |  | [optional]
**sport_key** | Option<**String**> | Sport external key (e.g. 'american-football'). Alternative to sport_id. If both are provided, sport_id takes precedence. | [optional]
**timezone** | Option<**String**> | IANA timezone for date interpretation (e.g. 'UTC', 'America/New_York', 'Europe/London') | [optional][default to UTC]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



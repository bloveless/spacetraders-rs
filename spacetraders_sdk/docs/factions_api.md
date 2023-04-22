# factions_api

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
**get-faction**](factions_api.md#get-faction) | **GET** /factions/{factionSymbol} | Get Faction
**get-factions**](factions_api.md#get-factions) | **GET** /factions | List Factions


# **get-faction**
> models::GetFaction200Response get-faction(ctx, faction_symbol)
Get Faction

View the details of a faction.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **faction_symbol** | **String**| The faction symbol | [default to "CGR".to_string()]

### Return type

[**models::GetFaction200Response**](get_faction_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-factions**
> models::GetFactions200Response get-factions(ctx, optional)
List Factions

List all discovered factions in the game.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **page** | **i32**| What entry offset to request | 
 **limit** | **i32**| How many entries to return per page | 

### Return type

[**models::GetFactions200Response**](get_factions_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


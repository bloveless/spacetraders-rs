# systems_api

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
**get-jump-gate**](systems_api.md#get-jump-gate) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate | Get Jump Gate
**get-market**](systems_api.md#get-market) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/market | Get Market
**get-shipyard**](systems_api.md#get-shipyard) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard | Get Shipyard
**get-system**](systems_api.md#get-system) | **GET** /systems/{systemSymbol} | Get System
**get-system-waypoints**](systems_api.md#get-system-waypoints) | **GET** /systems/{systemSymbol}/waypoints | List Waypoints
**get-systems**](systems_api.md#get-systems) | **GET** /systems | List Systems
**get-waypoint**](systems_api.md#get-waypoint) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol} | Get Waypoint


# **get-jump-gate**
> models::GetJumpGate200Response get-jump-gate(ctx, system_symbol, waypoint_symbol)
Get Jump Gate

Get jump gate details for a waypoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | 
  **waypoint_symbol** | **String**| The waypoint symbol | 

### Return type

[**models::GetJumpGate200Response**](get_jump_gate_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-market**
> models::GetMarket200Response get-market(ctx, system_symbol, waypoint_symbol)
Get Market

Retrieve imports, exports and exchange data from a marketplace. Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold. Send a ship to the waypoint to access trade good prices and recent transactions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | 
  **waypoint_symbol** | **String**| The waypoint symbol | 

### Return type

[**models::GetMarket200Response**](get_market_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-shipyard**
> models::GetShipyard200Response get-shipyard(ctx, system_symbol, waypoint_symbol)
Get Shipyard

Get the shipyard for a waypoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | 
  **waypoint_symbol** | **String**| The waypoint symbol | 

### Return type

[**models::GetShipyard200Response**](get_shipyard_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-system**
> models::GetSystem200Response get-system(ctx, system_symbol)
Get System

Get the details of a system.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | [default to "X1-OE".to_string()]

### Return type

[**models::GetSystem200Response**](get_system_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-system-waypoints**
> models::GetSystemWaypoints200Response get-system-waypoints(ctx, system_symbol, optional)
List Waypoints

Fetch all of the waypoints for a given system. System must be charted or a ship must be present to return waypoint details.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **system_symbol** | **String**| The system symbol | 
 **page** | **i32**| What entry offset to request | 
 **limit** | **i32**| How many entries to return per page | 

### Return type

[**models::GetSystemWaypoints200Response**](get_system_waypoints_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-systems**
> models::GetSystems200Response get-systems(ctx, optional)
List Systems

Return a list of all systems.

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

[**models::GetSystems200Response**](get_systems_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-waypoint**
> models::GetWaypoint200Response get-waypoint(ctx, system_symbol, waypoint_symbol)
Get Waypoint

View the details of a waypoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **system_symbol** | **String**| The system symbol | 
  **waypoint_symbol** | **String**| The waypoint symbol | 

### Return type

[**models::GetWaypoint200Response**](get_waypoint_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


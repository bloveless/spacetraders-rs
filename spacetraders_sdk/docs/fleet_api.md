# fleet_api

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
**create-chart**](fleet_api.md#create-chart) | **POST** /my/ships/{shipSymbol}/chart | Create Chart
**create-ship-ship-scan**](fleet_api.md#create-ship-ship-scan) | **POST** /my/ships/{shipSymbol}/scan/ships | Scan Ships
**create-ship-system-scan**](fleet_api.md#create-ship-system-scan) | **POST** /my/ships/{shipSymbol}/scan/systems | Scan Systems
**create-ship-waypoint-scan**](fleet_api.md#create-ship-waypoint-scan) | **POST** /my/ships/{shipSymbol}/scan/waypoints | Scan Waypoints
**create-survey**](fleet_api.md#create-survey) | **POST** /my/ships/{shipSymbol}/survey | Create Survey
**dock-ship**](fleet_api.md#dock-ship) | **POST** /my/ships/{shipSymbol}/dock | Dock Ship
**extract-resources**](fleet_api.md#extract-resources) | **POST** /my/ships/{shipSymbol}/extract | Extract Resources
**get-my-ship**](fleet_api.md#get-my-ship) | **GET** /my/ships/{shipSymbol} | Get Ship
**get-my-ship-cargo**](fleet_api.md#get-my-ship-cargo) | **GET** /my/ships/{shipSymbol}/cargo | Get Ship Cargo
**get-my-ships**](fleet_api.md#get-my-ships) | **GET** /my/ships | List Ships
**get-ship-cooldown**](fleet_api.md#get-ship-cooldown) | **GET** /my/ships/{shipSymbol}/cooldown | Get Ship Cooldown
**get-ship-nav**](fleet_api.md#get-ship-nav) | **GET** /my/ships/{shipSymbol}/nav | Get Ship Nav
**jettison**](fleet_api.md#jettison) | **POST** /my/ships/{shipSymbol}/jettison | Jettison Cargo
**jump-ship**](fleet_api.md#jump-ship) | **POST** /my/ships/{shipSymbol}/jump | Jump Ship
**navigate-ship**](fleet_api.md#navigate-ship) | **POST** /my/ships/{shipSymbol}/navigate | Navigate Ship
**orbit-ship**](fleet_api.md#orbit-ship) | **POST** /my/ships/{shipSymbol}/orbit | Orbit Ship
**patch-ship-nav**](fleet_api.md#patch-ship-nav) | **PATCH** /my/ships/{shipSymbol}/nav | Patch Ship Nav
**purchase-cargo**](fleet_api.md#purchase-cargo) | **POST** /my/ships/{shipSymbol}/purchase | Purchase Cargo
**purchase-ship**](fleet_api.md#purchase-ship) | **POST** /my/ships | Purchase Ship
**refuel-ship**](fleet_api.md#refuel-ship) | **POST** /my/ships/{shipSymbol}/refuel | Refuel Ship
**sell-cargo**](fleet_api.md#sell-cargo) | **POST** /my/ships/{shipSymbol}/sell | Sell Cargo
**ship-refine**](fleet_api.md#ship-refine) | **POST** /my/ships/{shipSymbol}/refine | Ship Refine
**transfer-cargo**](fleet_api.md#transfer-cargo) | **POST** /my/ships/{shipSymbol}/transfer | Transfer Cargo
**warp-ship**](fleet_api.md#warp-ship) | **POST** /my/ships/{shipSymbol}/warp | Warp Ship


# **create-chart**
> models::CreateChart201Response create-chart(ctx, ship_symbol)
Create Chart

Command a ship to chart the current waypoint.  Waypoints in the universe are uncharted by default. These locations will not show up in the API until they have been charted by a ship.  Charting a location will record your agent as the one who created the chart.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 

### Return type

[**models::CreateChart201Response**](create_chart_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create-ship-ship-scan**
> models::CreateShipShipScan201Response create-ship-ship-scan(ctx, ship_symbol)
Scan Ships

Activate your ship's sensor arrays to scan for ship information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::CreateShipShipScan201Response**](create_ship_ship_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create-ship-system-scan**
> models::CreateShipSystemScan201Response create-ship-system-scan(ctx, ship_symbol)
Scan Systems

Activate your ship's sensor arrays to scan for system information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::CreateShipSystemScan201Response**](create_ship_system_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create-ship-waypoint-scan**
> models::CreateShipWaypointScan201Response create-ship-waypoint-scan(ctx, ship_symbol)
Scan Waypoints

Activate your ship's sensor arrays to scan for waypoint information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::CreateShipWaypointScan201Response**](create_ship_waypoint_scan_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create-survey**
> models::CreateSurvey201Response create-survey(ctx, ship_symbol)
Create Survey

If you want to target specific yields for an extraction, you can survey a waypoint, such as an asteroid field, and send the survey in the body of the extract request. Each survey may have multiple deposits, and if a symbol shows up more than once, that indicates a higher chance of extracting that resource.  Your ship will enter a cooldown between consecutive survey requests. Surveys will eventually expire after a period of time. Multiple ships can use the same survey for extraction.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 

### Return type

[**models::CreateSurvey201Response**](create_survey_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dock-ship**
> models::DockShip200Response dock-ship(ctx, ship_symbol)
Dock Ship

Attempt to dock your ship at it's current location. Docking will only succeed if the waypoint is a dockable location, and your ship is capable of docking at the time of the request.  The endpoint is idempotent - successive calls will succeed even if the ship is already docked.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 

### Return type

[**models::DockShip200Response**](Dock_Ship_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **extract-resources**
> models::ExtractResources201Response extract-resources(ctx, ship_symbol, optional)
Extract Resources

Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The ship symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**| The ship symbol | 
 **extract_resources_request** | [**ExtractResourcesRequest**](ExtractResourcesRequest.md)|  | 

### Return type

[**models::ExtractResources201Response**](extract_resources_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-my-ship**
> models::GetMyShip200Response get-my-ship(ctx, ship_symbol)
Get Ship

Retrieve the details of your ship.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::GetMyShip200Response**](get_my_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-my-ship-cargo**
> models::GetMyShipCargo200Response get-my-ship-cargo(ctx, ship_symbol)
Get Ship Cargo

Retrieve the cargo of your ship.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 

### Return type

[**models::GetMyShipCargo200Response**](get_my_ship_cargo_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-my-ships**
> models::GetMyShips200Response get-my-ships(ctx, optional)
List Ships

Retrieve all of your ships.

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

[**models::GetMyShips200Response**](get_my_ships_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-ship-cooldown**
> models::GetShipCooldown200Response get-ship-cooldown(ctx, ship_symbol)
Get Ship Cooldown

Retrieve the details of your ship's reactor cooldown. Some actions such as activating your jump drive, scanning, or extracting resources taxes your reactor and results in a cooldown.  Your ship cannot perform additional actions until your cooldown has expired. The duration of your cooldown is relative to the power consumption of the related modules or mounts for the action taken.  Response returns a 204 status code (no-content) when the ship has no cooldown.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::GetShipCooldown200Response**](get_ship_cooldown_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-ship-nav**
> models::GetShipNav200Response get-ship-nav(ctx, ship_symbol)
Get Ship Nav

Get the current nav status of a ship.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The ship symbol | 

### Return type

[**models::GetShipNav200Response**](get_ship_nav_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jettison**
> models::Jettison200Response jettison(ctx, ship_symbol, optional)
Jettison Cargo

Jettison cargo from your ship's cargo hold.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **jettison_request** | [**JettisonRequest**](JettisonRequest.md)|  | 

### Return type

[**models::Jettison200Response**](jettison_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **jump-ship**
> models::JumpShip200Response jump-ship(ctx, ship_symbol, optional)
Jump Ship

Jump your ship instantly to a target system. Unlike other forms of navigation, jumping requires a unit of antimatter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **jump_ship_request** | [**JumpShipRequest**](JumpShipRequest.md)|  | 

### Return type

[**models::JumpShip200Response**](jump_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **navigate-ship**
> models::NavigateShip200Response navigate-ship(ctx, ship_symbol, optional)
Navigate Ship

Navigate to a target destination. The destination must be located within the same system as the ship. Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.  To travel between systems, see the ship's warp or jump actions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The ship symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**| The ship symbol | 
 **navigate_ship_request** | [**NavigateShipRequest**](NavigateShipRequest.md)|  | 

### Return type

[**models::NavigateShip200Response**](navigate_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **orbit-ship**
> models::OrbitShip200Response orbit-ship(ctx, ship_symbol)
Orbit Ship

Attempt to move your ship into orbit at it's current location. The request will only succeed if your ship is capable of moving into orbit at the time of the request.  The endpoint is idempotent - successive calls will succeed even if the ship is already in orbit.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 

### Return type

[**models::OrbitShip200Response**](Orbit_Ship_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch-ship-nav**
> models::GetShipNav200Response patch-ship-nav(ctx, ship_symbol, optional)
Patch Ship Nav

Update the nav data of a ship, such as the flight mode.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The ship symbol | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**| The ship symbol | 
 **patch_ship_nav_request** | [**PatchShipNavRequest**](PatchShipNavRequest.md)|  | 

### Return type

[**models::GetShipNav200Response**](get_ship_nav_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **purchase-cargo**
> models::PurchaseCargo201Response purchase-cargo(ctx, ship_symbol, optional)
Purchase Cargo

Purchase cargo.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **purchase_cargo_request** | [**PurchaseCargoRequest**](PurchaseCargoRequest.md)|  | 

### Return type

[**models::PurchaseCargo201Response**](Purchase_Cargo_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **purchase-ship**
> models::PurchaseShip201Response purchase-ship(ctx, optional)
Purchase Ship

Purchase a ship

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **purchase_ship_request** | [**PurchaseShipRequest**](PurchaseShipRequest.md)|  | 

### Return type

[**models::PurchaseShip201Response**](purchase_ship_201_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refuel-ship**
> models::RefuelShip200Response refuel-ship(ctx, ship_symbol)
Refuel Ship

Refuel your ship from the local market.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 

### Return type

[**models::RefuelShip200Response**](refuel_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **sell-cargo**
> models::SellCargo201Response sell-cargo(ctx, ship_symbol, optional)
Sell Cargo

Sell cargo.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **sell_cargo_request** | [**SellCargoRequest**](SellCargoRequest.md)|  | 

### Return type

[**models::SellCargo201Response**](Sell_Cargo_201_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **ship-refine**
> models::ShipRefine200Response ship-refine(ctx, ship_symbol, optional)
Ship Refine

Attempt to refine the raw materials on your ship. The request will only succeed if your ship is capable of refining at the time of the request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**| The symbol of the ship | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**| The symbol of the ship | 
 **ship_refine_request** | [**ShipRefineRequest**](ShipRefineRequest.md)|  | 

### Return type

[**models::ShipRefine200Response**](Ship_Refine_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **transfer-cargo**
> models::TransferCargo200Response transfer-cargo(ctx, ship_symbol, optional)
Transfer Cargo

Transfer cargo between ships.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **transfer_cargo_request** | [**TransferCargoRequest**](TransferCargoRequest.md)|  | 

### Return type

[**models::TransferCargo200Response**](Transfer_Cargo_200_Response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **warp-ship**
> models::NavigateShip200Response warp-ship(ctx, ship_symbol, optional)
Warp Ship

Warp your ship to a target destination in another system. Warping will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.  The returned response will detail the route information including the expected time of arrival. Most ship actions are unavailable until the ship has arrived at it's destination.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ship_symbol** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ship_symbol** | **String**|  | 
 **navigate_ship_request** | [**NavigateShipRequest**](NavigateShipRequest.md)|  | 

### Return type

[**models::NavigateShip200Response**](navigate_ship_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


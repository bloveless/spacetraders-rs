# contracts_api

All URIs are relative to *https://api.spacetraders.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
**accept-contract**](contracts_api.md#accept-contract) | **POST** /my/contracts/{contractId}/accept | Accept Contract
**deliver-contract**](contracts_api.md#deliver-contract) | **POST** /my/contracts/{contractId}/deliver | Deliver Contract
**fulfill-contract**](contracts_api.md#fulfill-contract) | **POST** /my/contracts/{contractId}/fulfill | Fulfill Contract
**get-contract**](contracts_api.md#get-contract) | **GET** /my/contracts/{contractId} | Get Contract
**get-contracts**](contracts_api.md#get-contracts) | **GET** /my/contracts | List Contracts


# **accept-contract**
> models::AcceptContract200Response accept-contract(ctx, contract_id)
Accept Contract

Accept a contract.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **contract_id** | **String**|  | 

### Return type

[**models::AcceptContract200Response**](accept_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deliver-contract**
> models::DeliverContract200Response deliver-contract(ctx, contract_id, optional)
Deliver Contract

Deliver cargo on a given contract.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **contract_id** | **String**| The ID of the contract | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **contract_id** | **String**| The ID of the contract | 
 **deliver_contract_request** | [**DeliverContractRequest**](DeliverContractRequest.md)|  | 

### Return type

[**models::DeliverContract200Response**](deliver_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **fulfill-contract**
> models::FulfillContract200Response fulfill-contract(ctx, contract_id)
Fulfill Contract

Fulfill a contract

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **contract_id** | **String**| The ID of the contract | 

### Return type

[**models::FulfillContract200Response**](fulfill_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-contract**
> models::GetContract200Response get-contract(ctx, contract_id)
Get Contract

Get the details of a contract by ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **contract_id** | **String**| The contract ID | 

### Return type

[**models::GetContract200Response**](get_contract_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-contracts**
> models::GetContracts200Response get-contracts(ctx, optional)
List Contracts

List all of your contracts.

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

[**models::GetContracts200Response**](get_contracts_200_response.md)

### Authorization

[AgentToken](../README.md#AgentToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


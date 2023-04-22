# Contract

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**faction_symbol** | **String** | The symbol of the faction that this contract is for. | 
**r#type** | **String** |  | 
**terms** | [***models::ContractTerms**](ContractTerms.md) |  | 
**accepted** | **bool** | Whether the contract has been accepted by the agent | [default to false]
**fulfilled** | **bool** | Whether the contract has been fulfilled | [default to false]
**expiration** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | The time at which the contract expires | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



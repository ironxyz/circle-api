# \WiresApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_wire_account**](WiresApi.md#create_business_wire_account) | **POST** /v1/businessAccount/banks/wires | Create a Wire bank account
[**get_business_wire_account**](WiresApi.md#get_business_wire_account) | **GET** /v1/businessAccount/banks/wires/{id} | Get a Wire bank account
[**get_business_wire_account_instructions**](WiresApi.md#get_business_wire_account_instructions) | **GET** /v1/businessAccount/banks/wires/{id}/instructions | Get Wire instructions
[**list_business_wire_accounts**](WiresApi.md#list_business_wire_accounts) | **GET** /v1/businessAccount/banks/wires | List all Wire bank accounts



## create_business_wire_account

> models::CreateBusinessWireAccountResponse create_business_wire_account(wire_creation_request)
Create a Wire bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wire_creation_request** | Option<[**WireCreationRequest**](WireCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessWireAccountResponse**](CreateBusinessWireAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_wire_account

> models::GetBusinessWireAccountResponse get_business_wire_account(id)
Get a Wire bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetBusinessWireAccountResponse**](GetBusinessWireAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_wire_account_instructions

> models::GetBusinessWireAccountInstructionsResponse get_business_wire_account_instructions(id, currency)
Get Wire instructions

Get the wire transfer instructions into the Circle bank account given your bank account id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**currency** | Option<**String**> | Queries beneficiary bank account currency. Default is USD. |  |

### Return type

[**models::GetBusinessWireAccountInstructionsResponse**](GetBusinessWireAccountInstructionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_wire_accounts

> models::ListBusinessWireAccountsResponse list_business_wire_accounts()
List all Wire bank accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBusinessWireAccountsResponse**](ListBusinessWireAccountsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


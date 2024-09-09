# \CbitApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_cbit_account**](CbitApi.md#create_business_cbit_account) | **POST** /v1/businessAccount/banks/cbit | Create a CBIT bank account
[**get_business_cbit_account**](CbitApi.md#get_business_cbit_account) | **GET** /v1/businessAccount/banks/cbit/{id} | Get a CBIT bank account
[**get_business_cbit_account_instructions**](CbitApi.md#get_business_cbit_account_instructions) | **GET** /v1/businessAccount/banks/cbit/{id}/instructions | Get CBIT instructions
[**list_business_cbit_accounts**](CbitApi.md#list_business_cbit_accounts) | **GET** /v1/businessAccount/banks/cbit | List all CBIT bank accounts.



## create_business_cbit_account

> models::CreateBusinessCbitAccountResponse create_business_cbit_account(cbit_fiat_account_creation_request)
Create a CBIT bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cbit_fiat_account_creation_request** | Option<[**CbitFiatAccountCreationRequest**](CbitFiatAccountCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessCbitAccountResponse**](CreateBusinessCbitAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_cbit_account

> models::GetBusinessCbitAccountResponse get_business_cbit_account(id)
Get a CBIT bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetBusinessCbitAccountResponse**](GetBusinessCbitAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_cbit_account_instructions

> models::ListBusinessCbitAccountInstructionsResponse get_business_cbit_account_instructions(id)
Get CBIT instructions

Get the CBIT transfer instructions into the Circle bank account given your bank account id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::ListBusinessCbitAccountInstructionsResponse**](ListBusinessCbitAccountInstructionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_cbit_accounts

> models::ListBusinessCbitAccountsResponse list_business_cbit_accounts()
List all CBIT bank accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBusinessCbitAccountsResponse**](ListBusinessCbitAccountsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


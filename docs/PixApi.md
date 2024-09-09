# \PixApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_pix_account**](PixApi.md#create_business_pix_account) | **POST** /v1/businessAccount/banks/pix | Create a PIX bank account
[**get_business_pix_account**](PixApi.md#get_business_pix_account) | **GET** /v1/businessAccount/banks/pix/{id} | Get a PIX bank account
[**get_business_pix_account_instructions**](PixApi.md#get_business_pix_account_instructions) | **GET** /v1/businessAccount/banks/pix/{id}/instructions | Get PIX instructions
[**list_business_pix_accounts**](PixApi.md#list_business_pix_accounts) | **GET** /v1/businessAccount/banks/pix | List all PIX bank accounts.



## create_business_pix_account

> models::CreateBusinessPixAccountResponse create_business_pix_account(pix_fiat_account_creation_request)
Create a PIX bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pix_fiat_account_creation_request** | Option<[**PixFiatAccountCreationRequest**](PixFiatAccountCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessPixAccountResponse**](CreateBusinessPixAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_pix_account

> models::GetBusinessPixAccountResponse get_business_pix_account(id)
Get a PIX bank account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetBusinessPixAccountResponse**](GetBusinessPixAccountResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_pix_account_instructions

> models::BusinessPixAccountInstructionsResponse get_business_pix_account_instructions(id)
Get PIX instructions

Get the PIX transfer instructions into the Circle bank account given your bank account id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::BusinessPixAccountInstructionsResponse**](BusinessPixAccountInstructionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_pix_accounts

> models::ListBusinessPixAccountsResponse list_business_pix_accounts()
List all PIX bank accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBusinessPixAccountsResponse**](ListBusinessPixAccountsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \BalancesApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_balances**](BalancesApi.md#list_balances) | **GET** /v1/balances | List all balances
[**list_business_balances**](BalancesApi.md#list_business_balances) | **GET** /v1/businessAccount/balances | List all balances



## list_balances

> models::ListBalancesResponse list_balances()
List all balances

Retrieves the balance of merchant funds that have settled and also of funds that have been sent for processing but have not yet settled.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBalancesResponse**](ListBalancesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_balances

> models::ListBusinessBalancesResponse list_business_balances()
List all balances

Retrieves the balance of funds that are available for use.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBusinessBalancesResponse**](ListBusinessBalancesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


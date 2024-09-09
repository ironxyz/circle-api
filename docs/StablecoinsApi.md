# \StablecoinsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_stablecoins**](StablecoinsApi.md#list_stablecoins) | **GET** /v1/stablecoins | List all stablecoins



## list_stablecoins

> models::ListStablecoinsResponse list_stablecoins()
List all stablecoins

Retrieves total circulating supply for supported stablecoins across all chains. This endpoint is rate limited to one call per minute (based on IP).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListStablecoinsResponse**](ListStablecoinsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


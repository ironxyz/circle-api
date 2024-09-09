# \EncryptionApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_key**](EncryptionApi.md#get_public_key) | **GET** /v1/encryption/public | Get public key



## get_public_key

> models::GetPublicKeyResponse get_public_key()
Get public key

Retrieves an RSA public key to be used in encrypting data sent to the API. Your public keys change infrequently, so we encourage you to cache this response value locally for a duration of 24 hours or more.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPublicKeyResponse**](GetPublicKeyResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


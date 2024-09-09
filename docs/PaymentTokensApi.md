# \PaymentTokensApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_token**](PaymentTokensApi.md#create_payment_token) | **POST** /v1/paymentTokens | Create a payment token



## create_payment_token

> models::CreatePaymentTokenResponse create_payment_token(payment_token_request)
Create a payment token

Convert a digital wallet (Apple Pay, Google Pay) token to a single-use payment token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_token_request** | Option<[**PaymentTokenRequest**](PaymentTokenRequest.md)> |  |  |

### Return type

[**models::CreatePaymentTokenResponse**](CreatePaymentTokenResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

